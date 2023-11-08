use everylog_rust_client::EverylogRustClient;
use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn test_notification() {
        // Start a mock server and get the URL
        let server_url = mockito::server_url();

        // Create a mock for the post request
        let mock = mock("POST", "/api/v1/log-entries")
            .match_header("Content-Type", "application/json")
            .match_header("Authorization", Matcher::Regex(r"^Bearer \w+$".to_string()))
            .with_status(200)
            .with_body(r#"{"success": true}"#)
            .create();

        let mut client = EverylogRustClient::new();
        client.setup(Some(json!({
            "api_key": "your_api_key",
            "projectId": "your_project_id",
            "everylog_url": server_url + "/api/v1/log-entries",
        })));

        let response = client.create_log_entry(Some(json!({
            "title": "Notification Title",
            "summary": "Notification Summary",
            "body": "Notification Body",
            "properties": [Some(json!({"author": "David"}))]
        })));

        match response {
            Ok(result) => {
                assert_eq!(result, json!({"success": true}));
                println!("Notification sent successfully: {:?}", result);
            }
            Err(err) => eprintln!("Error sending notification: {:?}", err),
        }

        // Assert that the mock request was called
        mock.assert();
    }
}
