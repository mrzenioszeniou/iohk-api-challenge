extern crate futures;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde_json;
extern crate simplelog;
extern crate structopt;

mod err;
mod wrapper;

use crate::err::Error;

use hyper::Uri;
use log::Level;
use simplelog::{Config, SimpleLogger};
use structopt::StructOpt;
use wrapper::Wrapper;

#[derive(StructOpt, Debug)]
#[structopt(name = "IOHK Test Assignment", author)]
struct Opt {
  #[structopt(
    short = "u",
    long = "uri",
    help = "The root URI for the API (e.g. http://ec2-3-65-182-233.eu-central-1.compute.amazonaws.com:8080)"
  )]
  uri: Uri,
  #[structopt(
    short = "t",
    long = "token",
    help = "A valid API token to use during the test calls (e.g. Oy3hfPoH45ze7Q)"
  )]
  token: String,
  #[structopt(
    short = "l",
    long = "level",
    default_value = "info",
    help = "Specify the log level (error, warn, info, debug, trace)"
  )]
  log_level: Level,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let opt = Opt::from_args();

  let _ = SimpleLogger::init(opt.log_level.to_level_filter(), Config::default());

  let mut wrapper = Wrapper::new(opt.uri, opt.token.clone());

  let mut success = true;

  // Simple health check before testing
  match wrapper.health_check() {
    Ok(()) => {
      info!("Health Check .. OK")
    }
    Err(e) => {
      warn!("Health Check .. FAILED ({})", e);
      std::process::exit(1);
    }
  }

  // Token Tests
  let tokens = vec![
    (opt.token.to_ascii_lowercase(), false),
    (opt.token.to_ascii_lowercase(), false),
    (opt.token.to_ascii_uppercase(), false),
    ("foobar".to_string(), false),
    ("".to_string(), false),
    (opt.token, true),
  ];

  for (token, ok) in tokens.into_iter() {
    match token_test(&mut wrapper, &token, ok) {
      Err(e) => {
        warn!("Token Tests .. FAILED ({})", e);
        success = false;
      }
      Ok(()) => {}
    }
  }

  if success {
    info!("Token Tests .. OK");
  }

  // ID Update Test
  match id_update_test(&mut wrapper) {
    Err(e) => {
      warn!("ID Update Test .. FAILED ({})", e);
      success = false;
    }
    Ok(()) => info!("ID Update Test .. OK"),
  }

  // Log Clearing Test
  match log_clear_test(&mut wrapper) {
    Err(e) => {
      warn!("Log Clearing Test .. FAILED ({})", e);
      success = false;
    }
    Ok(()) => info!("Log Clearing Test .. OK"),
  }

  if !success {
    std::process::exit(1);
  }

  Ok(())
}

///
/// Tests whether the provided token can be used to fetch the logs from the API
///
fn token_test(wrapper: &mut Wrapper, token: &str, success: bool) -> Result<(), String> {
  let initial_token = wrapper.get_token().to_string();

  wrapper.set_token(&token);

  let result = wrapper.fetch_logs();

  wrapper.set_token(&initial_token);

  if result.is_ok() != success {
    Err(format!(
      "Token '{}' should {}have been authenticated ",
      token,
      if success { "" } else { "not " },
    ))
  } else {
    Ok(())
  }
}

///
/// Tests whether the API supports updating of the fund ID
///
fn id_update_test(wrapper: &mut Wrapper) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let id = wrapper.fetch_fund_id()?;

  wrapper.update_fund_id((id + 1) % 42)?;

  if wrapper.fetch_fund_id()? != (id + 1) % 42 {
    Err(Box::from(Error::from("Fund ID was not updated")))
  } else {
    Ok(())
  }
}

///
/// Tests whether the API supports purging of the logs
///
fn log_clear_test(wrapper: &mut Wrapper) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let _ = wrapper.fetch_fund_id()?;

  if wrapper.fetch_logs()?.is_empty() {
    return Err(Box::from(Error::from(
      "API should have at least one log entry after query",
    )));
  }

  wrapper.clean_logs()?;

  if !wrapper.fetch_logs()?.is_empty() {
    return Err(Box::from(Error::from(
      "API should not have any log entries after clearing",
    )));
  }

  Ok(())
}
