# password

*Generate passwords.*

---

Basically pwgen but with simpler options (`-a` for alphabetic, `-s` for symbols, etc.). `password`
also doesn't include a newline when piping one password, meaning piping into clipboard works as
expected.

## Usage

```plaintext
% password --help
password 0.1.0
Kyle Clemens <git@kyleclemens.com>
Generate passwords

USAGE:
    password [FLAGS] [OPTIONS] <length> [amount]

FLAGS:
    -a, --alphabetic    include upper and lowercase alphabetic characters
    -h, --help          Prints help information
    -l, --lowercase     include lowercase characters
    -n, --numerals      include numerals
    -s, --symbols       include symbols
    -u, --uppercase     include uppercase characters
    -V, --version       Prints version information

OPTIONS:
    -c, --custom <custom>    provide characters to include

ARGS:
    <length>    the length of passwords to generate
    <amount>    the amount of passwords to generate [default: 1]

% password -ans 64
={'yU(vHn>>4\G}c6t01cH}<DV:anEPn^CD_dnN-Y's<X%STXAb/PU1h~l_Cjx!a
```
