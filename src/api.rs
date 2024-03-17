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

    pub fn get_host_count(&self, query: &str) -> Result<Value, Error> {
        let url = "https://api.shodan.io/shodan/host/count";
        let response = self.client.get(url)
            .query(&[("key", &self.api_key), ("query", &query.to_string())])
            .send()?;

        response.json()
    }

    pub fn search_hosts(&self, query: &str) -> Result<Value, Error> {
        let url = "https://api.shodan.io/shodan/host/search";
        let response = self.client.get(url)
            .query(&[("key", &self.api_key), ("query", &query.to_string())])
            .send()?;

        response.json()
    }

    pub fn get_ports(&self) -> Result<Value, Error> {
        let url = "https://api.shodan.io/shodan/ports";
        let response = self.client.get(url)
            .query(&[("key", &self.api_key)])
            .send()?;

        response.json()
    }

    pub fn get_protocols(&self) -> Result<Value, Error> {
        let url = "https://api.shodan.io/shodan/protocols";
        let response = self.client.get(url)
            .query(&[("key", &self.api_key)])
            .send()?;

        response.json()
    }

    pub fn get_search_tokens(&self, query: &str) -> Result<Value, Error> {
        let url = "https://api.shodan.io/shodan/host/search/tokens";
        let response = self.client.get(url)
            .query(&[("key", &self.api_key), ("query", &query.to_string())])
            .send()?;

        response.json()
    }
}
