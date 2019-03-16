use rand::{Rng, seq::SliceRandom};

use std::boxed::Box;

#[derive(Debug)]
pub struct GenerationOptions<'a> {
  pub lowercase: bool,
  pub uppercase: bool,
  pub numerals: bool,
  pub symbols: bool,
  pub custom: Option<&'a str>,
}

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERALS: &str = "0123456789";
const SYMBOLS: &str = "`~!@#$%^&*()_+-=[]\\{}|;':\",./<>?";

impl<'a> GenerationOptions<'a> {
  fn char_vec(&self) -> Vec<char> {
    let mut iter: Box<Iterator<Item = char>> = Box::new(std::iter::empty());

    if self.lowercase {
      iter = Box::new(iter.chain(LOWERCASE.chars()));
    }
    if self.uppercase {
      iter = Box::new(iter.chain(UPPERCASE.chars()));
    }
    if self.numerals {
      iter = Box::new(iter.chain(NUMERALS.chars()));
    }
    if self.symbols {
      iter = Box::new(iter.chain(SYMBOLS.chars()));
    }
    if let Some(ref s) = self.custom {
      iter = Box::new(iter.chain(s.chars()));
    }

    iter.collect()
  }

  pub fn is_populated(&self) -> bool {
    self.lowercase
      || self.uppercase
      || self.numerals
      || self.symbols
      || self.custom.map(|x| !x.is_empty()).unwrap_or(false)
  }

  pub fn is_empty(&self) -> bool {
    !self.is_populated()
  }

  #[allow(unused)]
  pub fn generate<R>(&self, rng: &mut R, length: usize) -> Option<String>
    where R: Rng,
  {
    self.passwords(rng, length).next()
  }

  pub fn passwords<'r, R>(&self, rng: &'r mut R, length: usize) -> PasswordGenerator<'r, R>
    where R: Rng,
  {
    let chars = self.char_vec();
    PasswordGenerator {
      chars,
      length,
      rng,
    }
  }
}

pub struct PasswordGenerator<'r, R> {
  chars: Vec<char>,
  length: usize,
  rng: &'r mut R,
}

impl<'r, R: Rng> Iterator for PasswordGenerator<'r, R> {
  type Item = String;

  fn next(&mut self) -> Option<Self::Item> {
    (0..self.length)
      .map(|_| self.chars.choose(self.rng).cloned())
      .collect()
  }
}
