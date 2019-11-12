use serde:: { Serialize };

use serde_json:: { Value };

use clap:: {ArgMatches };

use reqwest:: blocking:: { Client };
use reqwest:: { StatusCode, Error };

pub struct FsApi {
  email: String,
  password: String,
  vip: bool,
  token: String,
  session: String, // session
}

#[derive(Serialize, Debug)]
pub struct FsVipRequest<'a> {
  user_email: &'a str,
  password: &'a str,
  app_key: &'a str,
}

fn _create(email: String, password: String, vip: bool) -> FsApi {
  FsApi {
    email: email,
    password: password,
    token: String::from(""),
    session: String::from(""),
    vip: vip,
  }
}

pub fn make(matches: &ArgMatches) -> FsApi {
  let email = matches.value_of("email").unwrap();
  let password = matches.value_of("password").unwrap();
  let vip = matches.value_of("isvip");
  _create(String::from(email), String::from(password), vip.unwrap().parse().unwrap())
}

pub trait FsLogin {
  fn login(&mut self);
}



impl FsApi {
  pub fn login(&mut self) -> Result<(), Error> {
    if self.vip {
      self._login_vip()?
    } else {
      self._login_normal()?
    }

    Ok(())
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

    let status = res.status();
    let data = &res.text()?;
    let v: Value = serde_json::from_str(data.as_ref()).unwrap();

    match status {
      StatusCode::OK => {
        self.session = v["session_id"].as_str().unwrap().to_string();
        self.token = v["token"].as_str().unwrap().to_string();

        println!("{:?}", v);
      },
      s => {
        let code: u64 = v["code"].as_u64().unwrap();
        let msg: &str = v["msg"].as_str().unwrap();
        println!("Received response status: {:?}", s);
        println!("Code: {}", code);
        println!("Message: {}", msg);
      },
    }

    Ok(())
  }

  fn _login_normal(&mut self) -> Result<(), Error> {
    println!("Not implement yet");
    Ok(())
  }
}
