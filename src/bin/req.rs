use reqwest::Error;

use fshare;

fn main() -> Result<(), Error> {
  let matches = fshare::cmd::make();
  let api = &mut fshare::make(&matches);

  api.login();
  Ok(())
}