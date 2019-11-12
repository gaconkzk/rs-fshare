use serde:: { Deserialize, Serialize };

use clap:: {ArgMatches };

use reqwest:: blocking:: { Client };
use reqwest:: { Error };

#[derive(Debug)]
pub struct FsApi {
  email: String,
  password: String,
  vip: bool,
  session: Option<LoginResponse>,
}

#[derive(Deserialize, Serialize, Debug)]
struct LoginResponse {
  code: u64,
  msg: String,
  session_id: Option<String>,
  token: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FsVipRequest<'a> {
  user_email: &'a str,
  password: &'a str,
  app_key: &'a str,
}

fn _create(email: String, password: String, vip: bool) -> Result<FsApi, Error> {
  let mut api = FsApi {
    email: email,
    password: password,
    session: None,
    vip: vip,
  };

  api.login()?;

  Ok(api)
}

pub fn make(matches: &ArgMatches) -> Result<FsApi, Error> {
  let email = matches.value_of("email").unwrap();
  let password = matches.value_of("password").unwrap();
  let vip = matches.value_of("isvip").unwrap().parse().unwrap();

  _create(String::from(email), String::from(password), vip)
}

impl FsApi {
  fn login(&mut self) -> Result<(), Error> {
    if self.vip {
      self._login_vip()?
    } else {
      self._login_normal()?
    }

    Ok(())
  }

  pub fn get(&mut self, code: &str) -> Result<String, Error> {
    Ok("".to_string())
  }

  fn _login_vip(&mut self) -> Result<(), Error> {
    let request_url = format!("https://api.fshare.vn:443/api/user/login");

    let request = FsVipRequest {
      user_email: self.email.as_ref(),
      password: self.password.as_ref(),
      app_key: "L2S7R6ZMagggC5wWkQhX2+aDi467PPuftWUMRFSn",
    };

    let client = Client::new();

    let res = client.post(&request_url)
      .json(&request)
      .send()?;

    let data = &res.text()?;
    self.session = match serde_json::from_str(data.as_ref()) {
      Ok(val) => Some(val),
      _ => None,
    };

    println!("{:?}", self);

    Ok(())
  }

  fn _login_normal(&mut self) -> Result<(), Error> {
    println!("Not implement yet");
    Ok(())
  }
}
