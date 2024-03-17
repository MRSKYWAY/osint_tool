// data.rs

use serde_json::Value;

pub struct DataProcessor {}

impl DataProcessor {
    pub fn process_host_info(info: &Value) -> String {
        let mut result = String::new();

        // Extract IP address
        if let Some(ip) = info.get("ip_str") {
            result.push_str(&format!("IP Address: {}\n", ip));
        }

        // Extract port information
        if let Some(ports) = info.get("ports") {
            if let Some(ports_array) = ports.as_array() {
                if !ports_array.is_empty() {
                    result.push_str("Ports:\n");
                    for port in ports_array {
                        if let Some(port_num) = port.as_u64() {
                            result.push_str(&format!(" - {}\n", port_num));
                        }
                    }
                }
            }
        }

        // Extract other relevant information as needed
        // Example: Extract country information
        if let Some(country) = info.get("country_name") {
            result.push_str(&format!("Country: {}\n", country));
        }

        result
    }
}
