#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use failure::{bail, Error};
use rocket::request::Form;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

const BACKEND_URL: &str = "http://backend_url";

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct SlackRequest {
    token: String,
    team_id: String,
    team_domain: String,
    enterprise_id: String,
    enterprise_name: String,
    channel_id: String,
    channel_name: String,
    user_id: String,
    user_name: String,
    command: String,
    text: String,
    response_url: String,
    trigger_id: String,
    api_app_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct SlackResponse {
    status: String,
}

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
            _ => bail!("ERROR"),
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

#[post("/", data = "<req>")]
fn slack_request(req: Form<SlackRequest>) -> Json<SlackResponse> {
    let request: SlackRequest = req.into_inner();

    // Validate slack token here

    let command: Vec<&str> = request.text.split_ascii_whitespace().collect();

    let mut res: SlackResponse = SlackResponse {
        status: "".to_string(),
    };

    // correct number of arguments?
    if command.len() > 0 && command.len() < 5 {
        // List context
        if command[0] == "list" {
            res.status = format!("/timereport list")
        // Add context
        } else if command[0] == "add" {
            res.status = format!("/timereport add")
        // Delete context
        } else if command[0] == "delete" {
            res.status = format!("/timereport delete")
        // Error handling
        } else {
            res.status = format!("wrong number of arguments: {}", command.len())
        }
    }

    // validate reason
    if command.len() == 3 || command.len() == 4 {
        if let Ok(reason) = Reason::from_str(command[1]) {
            res.status = format!("{} is a valid reason.", command[1]);
        } else {
            res.status = format!("{} is an invalid reason.", command[1]);
        }
    } else {
        res.status = format!("Wrong number of args supplied {}", command.len());
    }

    // respond
    Json(res)
}

fn main() {
    start_server().launch();
}

fn start_server() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![slack_request])
}
