# everylog_rust_client

EveryLog provides a simple way to receive notifications for important application events that you choose.

## Installation

The distribution is hosted on a public repository like GitHub or GitLab, or on a package repository like crates.io at: '#'. To directly install the package run from your terminal::

    $ cargo install everylog_rust_client

# Usage

## Setup

This is to be set once globally (instantiated) from within the project, and used everywhere else

```rust
/// Sends a notification to EveryLog.
///
/// # Arguments
///
/// * `notify_options` - A Json object containing the options for the notification.

use everylog_rust_client::EverylogRustClient;
/// use serde_json::json;
///
/// 
fn main() {
    let mut client = EverylogRustClient::new();
    /// Sets up the client with the request arguments
    client.setup(Some(json!({
        "api_key": "your_api_key",
        "projectId": "your-project-id"
    })));
    /// notifies with notification options
    let response = client.notify(Some(json!({
        "title": "Notification Title",
        "summary": "Notification Summary",
        "body": "Notification Body",
        "tags": ["tag1", "tag2"],
        "properties": Some(json!({
            "key": 1,
            "another-key": 2
        })
    })));
    /// Handles response
    match response {
      Ok(result) => println!("Notification sent successfully: {:?}", result),
      Err(err) => eprintln!("Error sending notification: {:?}", err),
    }
}
 ```

## License

The package is available as open source under the terms of the [MIT License](LICENSE).

## Code of Conduct

Everyone interacting in the EveryLog python client project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](./CODE_OF_CONDUCT.md)