pub fn validate_token(s: &str) -> Result<&'static str, &'static str> {
    match s.len() {
        5 => Ok("valid token"),
        _ => Err("invalid token"),
    }
}
