use rand::rngs::EntropyRng;

use std::io::Write;

mod cli;
mod gen;

use self::gen::GenerationOptions;

fn main() {
  if let Err(e) = inner() {
    eprintln!("error: {}", e);
    std::process::exit(1);
  }
}

fn inner() -> Result<(), String> {
  let matches = cli::app().get_matches();

  let options = GenerationOptions {
    lowercase: matches.is_present("lowercase") || matches.is_present("alphabetic"),
    uppercase: matches.is_present("uppercase") || matches.is_present("alphabetic"),
    numerals: matches.is_present("numerals"),
    symbols: matches.is_present("symbols"),
    custom: matches.value_of("custom"),
  };

  if options.is_empty() {
    return Err("please specify some character groups to include (see --help)".into());
  }

  let length: usize = matches.value_of("length")
    .expect("required arg")
    .parse()
    .map_err(|e| format!("could not parse length as integer: {}", e))?;
  let amount: usize = matches.value_of("amount")
    .expect("default arg")
    .parse()
    .map_err(|e| format!("could not parse length as integer: {}", e))?;

  let is_tty = atty::is(atty::Stream::Stdout);
  let ending = if is_tty || amount > 1 { "\n" } else { "" };

  let mut rng = EntropyRng::default();

  let stdout = std::io::stdout();
  let mut lock = stdout.lock();

  for pw in options.passwords(&mut rng, length).take(amount) {
    write!(lock, "{}{}", pw, ending).map_err(|e| format!("could not write to stdout: {}", e))?;
  }

  Ok(())
}
