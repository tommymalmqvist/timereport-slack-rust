use slack;

#[cfg(test)]
mod tests {
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use slack::start_server;

    
    #[test]
    fn test_slack_request() {
        let client = Client::new(start_server()).unwrap();
        let mut response = client
            .post("/")
            .body("token=token&team_id=team_id&team_domain=team_domain&enterprise_id=enterprise_id&enterprise_name=enterprise_name&channel_id=channel_id&channel_name=channel_name&user_id=user_id&user_name=user_name&command=command&text=add+vacation+today&response_url=response_url&trigger_id=trigger_id&api_app_id=api_app_id")
            .header(ContentType::Form)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(
            response.body_string(),
            Some("{\"status\":\"vacation is a valid reason.\"}".into())
        );
    }

    #[test]
    fn test_slack_request_invalid_reason() {
        let client = Client::new(start_server()).unwrap();
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
