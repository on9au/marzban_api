# marzban_api
A simple async client which simplifies the interactions with the [Marzban](https://github.com/Gozargah/Marzban) panel API.
## Using
Either add it via Cargo:
```sh
$ cargo add marzban_api
```
or via Cargo.toml:
```toml
[dependencies]
marzban_api = "0.1.0"
```

## Examples
Simple example of using marzban_api:
```rust
use marzban_api::client::MarzbanAPIClient;
use marzban_api::models::auth::BodyAdminTokenApiAdminTokenPost;
use tokio;

#[tokio::main]
async fn main() {
    // Initialize the API client
    let base_url = "https://api.example.com"; // Replace with your actual base URL
    let api_client = MarzbanAPIClient::new(base_url);

    // Authentication, successful authentication will insert token
    // into MarzbanAPIClient, being used for every new API interaction.
    let auth = BodyAdminTokenApiAdminTokenPost {
        grant_type: Some("password".to_string()),
        username: "admin".to_string(), // Replace with your actual admin username
        password: "password".to_string(), // Replace with your actual admin password
        scope: "".to_string(),
        client_id: None,
        client_secret: None,
    };

    match api_client.authenticate(&auth).await {
        Ok(_) => println!("Authentication successful"),
        Err(e) => eprintln!("Authentication failed: {:?}", e),
    }

    // Get current admin
    match api_client.get_current_admin().await {
        Ok(admin) => println!("Current admin: {:?}", admin),
        Err(e) => eprintln!("Failed to get current admin: {:?}", e),
    }
}
```

## Features
- Async API Client from Reqwest
- Error handling
- Full support for all Marzban API endpoints

## Contributing
Contributions are welcome. Please fork the repository and submit a pull request for review.

## License
This project is licensed under the MIT license. See the [LICENSE](./LICENSE) file for more details.

## Acknowledgements
- [Marzban](https://github.com/Gozargah/Marzban) for providing the API.

## Additional Information
For details on the Marzban API and schema, please refer to the [Marzban API Documentation](https://github.com/Gozargah/Marzban)