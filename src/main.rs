use everylog_rust_client::EveryLogRustClient;
use serde_json::json;

fn main() {
    let mut client = EveryLogRustClient::new();
    client.setup(Some(json!({
        "api_key": "api_keyd",
        "projectId": "test-rust"
    })));

    let response = client.notify(Some(json!({
        "title": "Notification Title",
        "summary": "Notification Summary",
        "body": "Notification Body",
        "push": true,
        "tags": ["tag1", "tag2"],
        "properties": Some(json!({
            "key": 1,
            "another-key": 2
        }))
    })));

    match response {
      Ok(result) => println!("Notification sent successfully: {:?}", result),
      Err(err) => eprintln!("Error sending notification: {:?}", err),
    }
}
