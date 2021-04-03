use crate::err::Error;
use futures::executor::block_on;
use hyper::{body::to_bytes, client::HttpConnector};
use hyper::{Body, Client, Request, Uri};
use serde_json::Value;

/// Wrapper structure that can interact with the API
pub struct Wrapper {
  client: Client<HttpConnector>,
  token: String,
  root_uri: Uri,
}

impl Wrapper {
  pub fn new(root_uri: Uri, token: String) -> Self {
    Self {
      client: Client::new(),
      token,
      root_uri,
    }
  }

  pub fn get_token(&self) -> &str {
    &self.token
  }

  pub fn set_token(&mut self, token: &str) {
    self.token = String::from(token);
  }

  ///
  /// Makes simple health check request to the API
  ///
  pub fn health_check(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri = format!("{}api/health", self.root_uri).parse::<Uri>()?;

    let request = Request::builder()
      .method("GET")
      .uri(uri)
      .body(Body::empty())
      .expect("INTERNAL ERROR: Couldn't build request");

    let response = block_on(self.client.request(request))?;

    if !response.status().is_success() {
      return Err(Box::from(Error::from(format!(
        "Health check failed: STATUS {:?}",
        response.status()
      ))));
    }

    Ok(())
  }

  ///
  /// Fetches logs from the API
  ///
  pub fn fetch_logs(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let uri = format!("{}api/control/logs/get", self.root_uri).parse::<Uri>()?;

    let request = Request::builder()
      .method("GET")
      .uri(uri.clone())
      .header("API-Token", &self.token)
      .body(Body::empty())
      .expect("INTERNAL ERROR: Couldn't build request");

    let mut response = block_on(self.client.request(request))?;

    if !response.status().is_success() {
      return Err(Box::from(Error::from(format!(
        "Failed to fetch logs: STATUS {:?}",
        response.status()
      ))));
    }

    let body_bytes = block_on(to_bytes(response.body_mut()))?.to_vec();
    let logs: Vec<String> = serde_json::from_str(&String::from_utf8(body_bytes)?)?;

    Ok(logs)
  }

  ///
  /// Cleans logs on the API
  ///
  pub fn clean_logs(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri = format!("{}api/control/logs/clear", self.root_uri).parse::<Uri>()?;

    let request = Request::builder()
      .method("POST")
      .uri(uri)
      .header("API-Token", &self.token)
      .body(Body::empty())
      .expect("INTERNAL ERROR: Couldn't build request");

    let response = block_on(self.client.request(request))?;

    if !response.status().is_success() {
      return Err(Box::from(Error::from(format!(
        "Failed to clear logs: STATUS {:?}",
        response.status()
      ))));
    }

    Ok(())
  }

  ///
  /// Fetches the fund ID from the API
  ///
  pub fn fetch_fund_id(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let uri = format!("{}api/v0/fund", self.root_uri).parse::<Uri>()?;

    let request = Request::builder()
      .method("GET")
      .uri(uri.clone())
      .body(Body::empty())
      .expect("INTERNAL ERROR: Couldn't build request");

    let mut response = block_on(self.client.request(request))?;

    if !response.status().is_success() {
      return Err(Box::from(Error::from(format!(
        "Failed to fetch fund ID: STATUS {:?}",
        response.status()
      ))));
    }

    let body_bytes = block_on(to_bytes(response.body_mut()))?.to_vec();
    let body: Value = serde_json::from_str(&String::from_utf8(body_bytes)?)?;

    Ok(body["id"].to_string().parse::<usize>()?)
  }

  ///
  /// Updates the fund ID on the API
  ///
  pub fn update_fund_id(&self, id: usize) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let uri = format!("{}api/control/command/fund/id/{}", self.root_uri, id).parse::<Uri>()?;

    let request = Request::builder()
      .method("POST")
      .uri(uri)
      .header("API-Token", &self.token)
      .body(Body::empty())
      .expect("INTERNAL ERROR: Couldn't build request");

    let response = block_on(self.client.request(request))?;

    if !response.status().is_success() {
      return Err(Box::from(Error::from(format!(
        "Failed to update fund ID: STATUS {:?}",
        response.status()
      ))));
    }

    Ok(())
  }
}
