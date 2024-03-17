pub mod api_client {
    use reqwest::{blocking::Client, Error};
    use serde_json::Value;

    pub struct ApiClient {
        api_key: String,
        client: Client,
    }

    impl ApiClient {
        pub fn new(api_key: &str) -> Self {
            ApiClient {
                api_key: api_key.to_string(),
                client: Client::new(),
            }
        }

        pub fn get_host_info(&self, ip_address: &str) -> Result<Value, Error> {
            let url = format!("https://api.shodan.io/shodan/host/{}", ip_address);
            let response = self.client.get(&url)
                .query(&[("key", &self.api_key)])
                .send()?;

            response.json()
        }
    }
}
