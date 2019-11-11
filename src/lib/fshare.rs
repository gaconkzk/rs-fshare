use serde:: { Serialize };

use clap:: {ArgMatches };

use reqwest:: blocking:: { Client };
use reqwest:: { Error };

use std::collections::HashMap;

pub struct FsApi {
  email: String,
  password: String,
  token: &'static str,
  session: &'static str, // session
}

#[derive(Serialize, Debug)]
pub struct FsRequest {
  user_email: String,
  password: String,
  app_key: String,
}

fn _create(email: String, password: String) -> FsApi {
  FsApi {
    email: email,
    password: password,
    token: "",
    session: "",
  }
}

pub fn make(matches: &ArgMatches) -> FsApi {
  let email = matches.value_of("email").unwrap();
  let password = matches.value_of("password").unwrap();

  _create(String::from(email), String::from(password))
}

impl FsApi {
  pub fn login(&mut self) -> Result<(), Error> {
    let request_url = format!("https://api.fshare.vn:443/api/user/login");

    let mut map = HashMap::new();
    map.insert("user_email", self.email.as_ref());
    map.insert("password", self.password.as_ref());
    map.insert("app_key", "L2S7R6ZMagggC5wWkQhX2+aDi467PPuftWUMRFSn");

    let client = Client::new();

    let res = client.post(&request_url)
      .json(&map)
      // .header("User-Agent", "okhttp/3.6.0")
      .send()?.text()?;

    println!("{:?}", res);

    Ok(())
    // let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    //                           owner = "gaconkzk",
    //                           repo = "buom");
    // println!("{}", request_url);
    // let mut response = reqwest::get(&request_url)?;

    // let users: Vec<User> = response.json()?;
    // println!("{:?}", users);
  }
}
