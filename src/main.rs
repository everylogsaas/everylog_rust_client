use everylog_rust_client::EveryLogRustClient;
use serde_json::json;

fn main() {
    let mut client = EveryLogRustClient::new();
    client.setup(Some(json!({
        "api_key": "api_key",
        "projectId": "project_id"
    })));

    let response = client.notify(Some(json!({
        "title": "Notification Title",
        "summary": "Notification Summary",
        "body": "Notification Body"
    })));

    match response {
      Ok(result) => println!("Notification sent successfully: {:?}", result),
      Err(err) => eprintln!("Error sending notification: {:?}", err),
    }
}
