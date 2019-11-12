use clap::{ App, Arg, ArgMatches };

pub fn make<'a>() -> ArgMatches<'a> {
  let matches = App::new("FShare Tool")
        .version("1.0")
        .author("gaconkzk <gaconkzk@gmail.com>")
        .about("fshare link")
        .arg(
          Arg::with_name("email")
            .short("e")
            .long("email")
            .env("VIP_MAIL")
            .help("fshare email")
            .hide_env_values(true)
            .takes_value(true)
        )
        .arg(
          Arg::with_name("password")
            .short("p")
            .long("password")
            .env("VIP_PASS")
            .help("fshare password")
            .hide_env_values(true)
            .takes_value(true)
        )
        .arg(
          Arg::with_name("isvip")
            .short("i")
            .long("vip")
            .help("vip mode or normal user")
            .possible_values(&["true", "false"])
            .default_value("false")
        )
        .arg(
          Arg::with_name("code")
            .short("c")
            .long("code")
            .help("fshare code")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

  return matches;
}
