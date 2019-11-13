use crate::Error;

use serde:: { Deserialize, Serialize };

use clap:: {ArgMatches };

use reqwest:: blocking:: { Client };
use reqwest:: { header, StatusCode };

use soup::prelude::*;

#[derive(Debug)]
pub struct FsApi {
  email: String,
  password: String,
  vip: bool,
  csrf_token: String,
  session: Option<LoginResponse>,
  _client: Client,
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

#[derive(Serialize, Debug)]
pub struct FsRequest<'a> {
  #[serde(rename = "_csrf-app")]
  _csrf_app: &'a str,
  #[serde(rename = "LoginForm[email]")]
  user_email: &'a str,
  #[serde(rename = "LoginForm[password]")]
  password: &'a str,
  #[serde(rename = "LoginForm[rememberMe]")]
  remember_me: u8,
}

fn _create(email: String, password: String, vip: bool) -> Result<FsApi, Error> {

  let mut api = FsApi {
    csrf_token: String::new(),
    email: email,
    password: password,
    session: None,
    vip: vip,
    _client: Client::builder()
      .cookie_store(true)
      .build()?,
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
      user_email: &self.email,
      password: &self.password,
      app_key: "L2S7R6ZMagggC5wWkQhX2+aDi467PPuftWUMRFSn",
    };

    let res = self._client.post(&request_url)
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
    let request_url = format!("https://www.fshare.vn/site/login");

    let csrf_token = self._csrf_token();
    match csrf_token {
      Ok(Some(val)) => {
        self.csrf_token = val;
        let request = FsRequest {
          _csrf_app: &self.csrf_token,
          user_email: &self.email,
          password: &self.password,
          remember_me: 1,
        };

        let client = Client::new();
        let res = client.post(&request_url)
        .form(&request)
        .send()?;

        let status = &res.status();

        match *status {
          StatusCode::OK => {
            let data = &res.text()?;
            println!("logged in. {:?}", data);
            // self.session = match serde_json::from_str(data.as_ref()) {
            //   Ok(val) => Some(val),
            //   _ => None,
            // };
          },
          _ => {
            println!("{:?}", res);
            return Err(Error::ReqwestError(format!("Error. Status code: {}",status)))
          }
        };

        Ok(())
      },
      _ => Err(Error::CsrfNotFound),
    }
  }

  fn _csrf_token(&self) -> Result<Option<String>, Error> {
    let request_url = format!("https://www.fshare.vn/site/login");

    let response = self._client.get(&request_url).send()?;

    let cookies = response.cookies();
    cookies.for_each(|c| {
      println!("{:?}", c);
    });

    let html = response.text()?;
    let soup = Soup::new(&html);

    Ok(soup.tag("meta").attr("name", "csrf-token").find().and_then(|m| m.get("content")))
  }

}
