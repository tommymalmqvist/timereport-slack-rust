#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct SlackRequest {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub enterprise_id: String,
    pub enterprise_name: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub api_app_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackResponse {
    pub status: String,
}
