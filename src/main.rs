mod parse {
    use super::MAX_LENGTH;

    use std::io::{self, BufRead};
    use std::num::ParseIntError;

    #[derive(Debug)]
    pub enum Error {
        Exhausted,
        Io(io::Error),
        InvalidInt(ParseIntError),
        InvalidDomain(u32, u32),
        InvalidPiece(String),
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::Io(err)
        }
    }
    impl From<ParseIntError> for Error {
        fn from(err: ParseIntError) -> Self {
            Error::InvalidInt(err)
        }
    }

    pub fn two_u32_max_10k(
        input: &mut impl BufRead,
        buf: &mut String,
    ) -> Result<(u32, u32), Error> {
        buf.clear();
        if input.read_line(buf)? == 0 {
            return Err(Error::Exhausted);
        }

        let mut splits = buf.split_whitespace();
        match (splits.next(), splits.next()) {
            (Some(w), Some(l)) => Ok((
                w.parse()
                    .map_err(Error::from)
                    .and_then(|v| max_of(v, MAX_LENGTH))?,
                l.parse()
                    .map_err(Error::from)
                    .and_then(|v| max_of(v, MAX_LENGTH))?,
            )),
            _ => Err(Error::InvalidPiece(buf.to_owned())),
        }
    }

    pub fn single_u32(input: &mut impl BufRead, buf: &mut String) -> Result<u32, Error> {
        buf.clear();
        if input.read_line(buf)? == 0 {
            Err(Error::Exhausted)
        } else {
            buf.trim_right().parse().map_err(Error::from)
        }
    }
    pub fn max_of(v: u32, max: u32) -> Result<u32, Error> {
        if v > max || v == 0 {
            Err(Error::InvalidDomain(v, max))
        } else {
            Ok(v)
        }
    }
}

use std::io::{stdin, stdout, BufReader, Write};
pub const MAX_LENGTH: u32 = 10_000;

use parse::max_of;

fn main() -> Result<(), parse::Error> {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let mut reader = BufReader::new(stdin_lock);

    let mut line_buf = String::new();

    let cake_width =
        parse::single_u32(&mut reader, &mut line_buf).and_then(|v| max_of(v, MAX_LENGTH))?;
    let num_pieces =
        parse::single_u32(&mut reader, &mut line_buf).and_then(|v| max_of(v, 5_000_000))?;

    let mut area_so_far = 0_u64;
    for _ in 0..num_pieces {
        let (w, l) = parse::two_u32_max_10k(&mut reader, &mut line_buf)?;
        area_so_far += (w * l) as u64;
    }
    let cake_length = area_so_far / cake_width as u64;
    writeln!(stdout(), "{}", cake_length)?;
    Ok(())
}
