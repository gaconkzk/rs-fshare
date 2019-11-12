use reqwest::Error;

use std;
use exitcode;

use fshare;

use dotenv::dotenv;

fn main() -> Result<(), Error> {
  dotenv().ok();

  let config = fshare::cmd::make();
  let vip: bool = config.value_of("isvip").unwrap().parse().unwrap();
  if !vip {
    println!("Not implement for non-VIP!");
    std::process::exit(exitcode::DATAERR);
  }

  // go
  let api = &mut fshare::make(&config)?;
  let fscode = config.value_of("code").unwrap();
  let location = api.get(fscode)?;
  println!("Location: {}", location);

  Ok(())
}
