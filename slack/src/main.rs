#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use api;
use rocket::request::Form;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

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

struct SlackResponse {
    status: String,
}

enum ActionType {
    Add,
    Delete,
    List,
}

#[derive(Serialize, Deserialize, Debug)]
struct Add {
    from: String,
    to: String,
    reason: Reason,
    hours: f32,
}

impl Add {
    fn new(s: &str) -> Add {
        // validate
        let v: Vec<&str> = s.split(' ').collect();
        if v[0] == "add" {
            Add {
                from: "now",
                to: "then",
                reason: Vacation,
                hours: 8.0,
            }
        }
    }
}
}}

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

#[post("/", data = "<req>")]
fn slack_request(req: Form<SlackRequest>) -> Json<SlackReponse> {
    let r = Json(req.into_inner());
    if let add: Add::new(r.text) {
        SlackResponse {
            status: "ok"
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![slack_request]).launch();
}
