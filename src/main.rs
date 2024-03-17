mod api;
mod data;

use api::ApiClient;
use data::DataProcessor;
use std::env;
fn main() {
    let api_key = match env::var("SHODAN_API_KEY") {
        Ok(val) => val,
        Err(_e) => {
            eprintln!("SHODAN_API_KEY environment variable is not set");
            return;
        },
    };

    let client = ApiClient::new(&api_key);

    // Get host info
    match client.get_host_info("8.8.8.8") {
        Ok(info) => {
            let processed_info = DataProcessor::process_host_info(&info);
            println!("Processed Host Info: {}", processed_info);
            println!("API Response: {:#?}", info);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get host count
    match client.get_host_count("apache") {
        Ok(count) => println!("Host Count: {:#?}", count),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Search hosts
    match client.search_hosts("apache") {
        Ok(hosts) => println!("Hosts: {:#?}", hosts),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get ports
    match client.get_ports() {
        Ok(ports) => println!("Ports: {:#?}", ports),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get protocols
    match client.get_protocols() {
        Ok(protocols) => println!("Protocols: {:#?}", protocols),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Get search tokens
    match client.get_search_tokens("apache") {
        Ok(tokens) => println!("Search Tokens: {:#?}", tokens),
        Err(e) => eprintln!("Error: {}", e),
    }
}
