use super::Reason;

pub fn validate_token(s: &str) -> Result<(), String> {
    match s.len() {
        5 => Ok(()),
        _ => Err(format!("{} is not a valid token", s)),
    }
}

pub fn validate_reason(s: &str) -> Result<(), String> {
    if let Ok(_) = Reason::from_str(s) {
        Ok(())
    } else {
        Err(format!("reason {} is not valid", s))
    }
}

#[cfg(test)]
mod test {
    use super::validate_reason;
    #[test]
    fn test_invalid_reason() {
        let s: &str = "invalid";
        assert_eq!(
            validate_reason(s),
            Err(format!("reason {} is not valid", s).into())
        );
    }

    #[test]
    fn test_valid_reason() {
        let valid: &str = "vacation";
        assert_eq!(validate_reason(valid), Ok(()));
    }
}
