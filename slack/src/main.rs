#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_slog;

#[macro_use]
extern crate slog;

#[macro_use]
extern crate sloggers;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate failure_derive;

use failure::{bail, Error};
use serde::{Deserialize, Serialize};

use rocket::request::Form;
use rocket_contrib::json::Json;

use rocket_slog::SlogFairing;
use sloggers::{
    terminal::{Destination, TerminalLoggerBuilder},
    types::Severity,
    Build,
};

mod model;
use model::{SlackRequest, SlackResponse};

mod utils;
use utils::validate_token;

const BACKEND_URL: &str = "http://backend_url";

#[derive(Serialize, Deserialize, Debug)]
struct Add {
    from: String,
    to: String,
    reason: Reason,
    hours: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Delete {
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum Reason {
    Sick,
    Vacation,
    Internal,
    ChildCare,
}

impl Reason {
    pub fn from_str(s: &str) -> Result<Reason, Error> {
        match s {
            "sick" => Ok(Reason::Sick),
            "vacation" => Ok(Reason::Vacation),
            "internal" => Ok(Reason::Internal),
            "childcare" => Ok(Reason::ChildCare),
            _ => bail!("Error: {s} is not a valid reason"),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Reason::Sick => "sick",
            Reason::Vacation => "vacation",
            Reason::Internal => "internal",
            Reason::ChildCare => "childcare",
        }
    }
}

enum Actions {
    Add,
    List,
    Delete,
}

impl Actions {
    pub fn from_str(s: &str) -> Result<Actions, Error> {
        match s {
            "add" => Ok(Actions::Add),
            "list" => Ok(Actions::List),
            "delete" => Ok(Actions::Delete),
            _ => bail!("Error: {} is not a valid action"),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            Actions::Add => "add",
            Actions::List => "list",
            Actions::Delete => "delete",
        }
    }
}

#[post("/", data = "<req>")]
fn slack_request(req: Form<SlackRequest>) -> Json<SlackResponse> {
    let mut res: SlackResponse = SlackResponse {
        status: "".to_string(),
    };
    let request: SlackRequest = req.into_inner();

    let validate_slack_token = validate_token(&request.token);

    match validate_slack_token {
        // should log here
        Ok(_) => println!("token valid"),
        Err(e) => {
            res.status = format!("Error: {}", e);
            return Json(res);
        }
    }

    let command: Vec<&str> = request.text.split_ascii_whitespace().collect();

    if command.len() == 0 {
        res.status = format!("{} is a valid reason", command[0]);
        return Json(res);
    }

    // context
    if command.len() == 3 || command.len() == 4 {
        if let Ok(_) = Reason::from_str(command[1]) {
            res.status = format!("{} is a valid reason.", command[1]);
        } else {
            res.status = format!("{} is an invalid reason.", command[1]);
        }
    } else {
        res.status = format!("Wrong number of args supplied {}", command.len());
    }

    Json(res)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    start_server().launch();

    Ok(())
}

pub fn start_server() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![slack_request])
}

#[cfg(test)]
mod test {
    use super::{rocket, start_server};
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_invalid_token() {
        let client = Client::new(start_server()).expect("valid rocket instance");
        let mut response = client
            .post("/")
            .body("token=fail&team_id=team_id&team_domain=team_domain&enterprise_id=enterprise_id&enterprise_name=enterprise_name&channel_id=channel_id&channel_name=channel_name&user_id=user_id&user_name=user_name&command=command&text=add+vacation+today&response_url=response_url&trigger_id=trigger_id&api_app_id=api_app_id")
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(
            response.body_string(),
            Some("{\"status\":\"Error: fail is not a valid token\"}".into())
        );
    }

    #[test]
    fn test_slack_request_invalid_reason() {
        let client = Client::new(start_server()).expect("valid rocket instance");
        let mut response = client
            .post("/")
            .body("token=token&team_id=team_id&team_domain=team_domain&enterprise_id=enterprise_id&enterprise_name=enterprise_name&channel_id=channel_id&channel_name=channel_name&user_id=user_id&user_name=user_name&command=command&text=add+invalid+reason&response_url=response_url&trigger_id=trigger_id&api_app_id=api_app_id")
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(
            response.body_string(),
            Some("{\"status\":\"invalid is an invalid reason.\"}".into())
        );
    }
}
