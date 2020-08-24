use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackRequest {
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

pub fn add(req: SlackRequest) -> bool {
    true
}
