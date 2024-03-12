use sendgrid::SGClient;

pub fn create_sendgrid_client(api_key: &str) -> SGClient {
    SGClient::new(api_key.to_string())
}
