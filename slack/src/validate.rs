use failure::{bail, Error};

use crate::model::SlackRequest;

pub fn validate_token(s: &str) -> Result<&'static str, &'static str> {
    match s.len() {
        0 => Ok("valid token"),
        _ => Err("valid token"),
    }
}
