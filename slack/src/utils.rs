use super::Reason;

pub fn validate_token(s: &str) -> Result<&'static str, &'static str> {
    match s.len() {
        5 => Ok("valid token"),
        _ => Err("invalid token"),
    }
}

pub fn validate_reason(s: &str) -> Result<String, String> {
    if let Ok(_) = Reason::from_str(s) {
        Ok(format!("reason {} is valid", s))
    } else {
        Err(format!("reason {} is not valid", s))
    }
}

#[cfg(test)]
mod test {
    use super::validate_reason;
    #[test]
    fn test_invalid_reason() {
        let invalid: &str = "invalid";
        assert_eq!(
            validate_reason(invalid),
            Err("reason fail is not valid".into())
        );
    }

    #[test]
    fn test_valid_reason() {
        let valid: &str = "vacation";
        assert_eq!(
            validate_reason(valid),
            Ok("reason vacation is valid".into())
        );
    }
}
