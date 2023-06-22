use serde_json::json;
use serde_json::Value;
use std::error::Error;
use reqwest;

pub struct EveryLogRustClient {
    setup_defaults: Value,
    notify_defaults: Value,
    options: Option<Value>,
    notify_options: Option<Value>,
}

impl EveryLogRustClient {
    pub fn new() -> EveryLogRustClient {
        let setup_defaults = json!({
            "api_key": null,
            "projectId": null,
            "everylog_url": "https://api.everylog.io/api/v1/log-entries"
        });

        let notify_defaults = json!({
            "title": "Empty notification",
            "summary": "Empty summary",
            "body": "Empty body",
            "tags": [],
            "link": "",
            "push": false,
            "icon": "",
            "externalChannels": [],
            "properties": {},
            "groups": [],
        });

        EveryLogRustClient {
            setup_defaults,
            notify_defaults,
            options: None,
            notify_options: None,
        }
    }

    pub fn setup(&mut self, options: Option<Value>) -> &mut EveryLogRustClient {
        self.options = self.parse_options(options, &self.setup_defaults);
        self
    }

    pub fn notify(&mut self, notify_options: Option<Value>) -> Result<Value, Box<dyn Error>> {
        self.notify_options = self.parse_options(notify_options, &self.notify_defaults);

        let mut merged_options = self.notify_options.clone().unwrap();
        merged_options["projectId"] = self.options.as_ref().unwrap()["projectId"].clone();

        let client = reqwest::blocking::Client::new();
        let url = self.options.as_ref().unwrap()["everylog_url"].as_str().unwrap();
        let api_key = self.options.as_ref().unwrap()["api_key"].as_str().unwrap();

        let response = client
                .post(url)
                .json(&merged_options)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .send()?
                .json::<Value>()?;

        Ok(response)
    }

    fn parse_options(&self, options: Option<Value>, defaults_to_dup: &Value) -> Option<Value> {
        let mut result_parsed_options = defaults_to_dup.clone();

        if let Some(opts) = options {
            if let Some(defaults) = result_parsed_options.as_object_mut() {
                for (key, value) in opts.as_object().unwrap().iter() {
                    defaults.insert(key.to_owned(), value.clone());
                }
            }
        }

        Some(result_parsed_options)
    }
}
