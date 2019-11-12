use reqwest::Error;

use fshare;

use dotenv::dotenv;

fn main() -> Result<(), Error> {
  dotenv().ok();

  let config = fshare::cmd::make();
  let api = &mut fshare::make(&config)?;

  let location = api.get(config.value_of("code").unwrap())?;
  println!("Location: {}", location);

  Ok(())
}
