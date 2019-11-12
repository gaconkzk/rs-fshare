use reqwest::Error;

use fshare;

use dotenv::dotenv;

fn main() -> Result<(), Error> {
  dotenv().ok();

  let config = fshare::cmd::make();
  let api = &mut fshare::make(&config);

  api.login().unwrap_or_else(|error| {
    panic!("Error: {}", error);
  });

  Ok(())
}
