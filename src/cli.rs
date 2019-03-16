use clap::{App, Arg};

pub(crate) fn app<'a, 'b: 'a>() -> App<'a, 'b> {
  App::new(clap::crate_name!())
    .version(clap::crate_version!())
    .author(clap::crate_authors!())
    .about(clap::crate_description!())

    .arg(Arg::with_name("alphabetic")
      .short("a")
      .long("alphabetic")
      .alias("alpha")
      .help("include upper and lowercase alphabetic characters"))
    .arg(Arg::with_name("lowercase")
      .short("l")
      .long("lowercase")
      .alias("lower")
      .help("include lowercase characters"))
    .arg(Arg::with_name("uppercase")
      .short("u")
      .long("uppercase")
      .alias("upper")
      .help("include uppercase characters"))
    .arg(Arg::with_name("numerals")
      .short("n")
      .long("numerals")
      .help("include numerals"))
    .arg(Arg::with_name("symbols")
      .short("s")
      .long("symbols")
      .help("include symbols"))
    .arg(Arg::with_name("custom")
      .short("c")
      .long("custom")
      .help("provide characters to include")
      .takes_value(true))

    .arg(Arg::with_name("length")
      .help("the length of passwords to generate")
      .validator(is_numeric)
      .required(true))
    .arg(Arg::with_name("amount")
      .help("the amount of passwords to generate")
      .validator(is_numeric)
      .default_value("1"))
}

fn is_numeric(s: String) -> Result<(), String> {
  if s.chars().all(|x| x.is_digit(10)) {
    Ok(())
  } else {
    Err("string must be numeric".into())
  }
}
