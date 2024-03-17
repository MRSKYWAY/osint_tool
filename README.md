```markdown
# Shodan OSINT Tool

This is a simple OSINT (Open Source Intelligence) tool written in Rust that uses the Shodan API to gather information about hosts.

## Project Structure

The project is organized into the following modules:

- `main.rs`: This is the entry point of the application.
- `api.rs`: This module contains the `ApiClient` struct and its associated functions for interacting with the Shodan API.
- `data.rs`: This module contains the `DataProcessor` struct and its associated function for processing the JSON response from the API.

## Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/shodan-osint-tool.git
cd shodan-osint-tool
```

2. Set your Shodan API key as an environment variable:

```bash
export SHODAN_API_KEY=your_api_key
```

3. Run the program:

```bash
cargo run
```

## Usage

The tool currently supports the following Shodan API endpoints:

- Get host information
- Get host count
- Search hosts
- Get ports
- Get protocols
- Get search tokens

The results are printed to the console.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.


