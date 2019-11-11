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
            .value_name("EMAIL")
            .help("fshare email")
            .takes_value(true)
            .required(true)
        )
        .arg(
          Arg::with_name("password")
            .short("p")
            .long("password")
            .value_name("PASSWORD")
            .help("fshare password")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

  return matches;
}
