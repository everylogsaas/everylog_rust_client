use everylog_rust_client::EverylogRustClient;
use serde_json::json;

fn main() {
    let mut client = EverylogRustClient::new();
    client.setup(Some(json!({
        "api_key": "api_key",
        "projectId": "testingrustclient",
        "everylog_url": "https://api.everylog.devdemo.it/api/v1/log-entries"
    })));

    let response = client.create_log_entry(Some(json!({
        "title": "Notification Title",
        "summary": "Notification Summary",
        "body": "Notification Body",
        "push": true,
        "tags": ["tag1", "tag2"],
        "properties": [Some(json!({
            "key": '1',
            "another-key": '2'
        }))]
    })));

    match response {
      Ok(result) => println!("Notification sent successfully: {:?}", result),
      Err(err) => eprintln!("Error sending notification: {:?}", err),
    }
}
