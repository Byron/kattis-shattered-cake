mod parse {
    use std::io;

    #[derive(Debug)]
    pub enum Error {
        Exhausted,
        Io(io::Error),
        InvalidDomain(u32, u32),
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::Io(err)
        }
    }

    const DIGIT_POWER: [u32; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];

    #[inline(always)]
    pub fn two_digits(input: &[u8], max_value: u32) -> Result<(u32, u32), Error> {
        let mut splits = input.split(|b| *b == b' ');
        match (splits.next(), splits.next()) {
            (Some(w), Some(l)) => Ok((single_digit(w, max_value)?, single_digit(l, max_value)?)),
            _ => Err(Error::Exhausted),
        }
    }
    #[inline(always)]
    pub fn single_digit(input: &[u8], max_value: u32) -> Result<u32, Error> {
        let res = input
            .iter()
            .rev()
            .map(|b| b - b'0')
            .enumerate()
            .fold(0, |acc, (index, b10)| acc + DIGIT_POWER[index] * b10 as u32);
        if res > max_value {
            Err(Error::InvalidDomain(res, max_value))
        } else {
            Ok(res)
        }
    }
}

use parse::Error;
use std::io::{stdin, stdout, Read, Write};
pub const MAX_LENGTH: u32 = 10_000;
pub const MAX_PIECES: u32 = 5_000_000;

fn main() -> Result<(), Error> {
    let mut buf = Vec::with_capacity(1024 * 1024);
    stdin().read_to_end(&mut buf)?;
    let mut lines = buf.split(|b| *b == b'\n');
    let cake_width = parse::single_digit(lines.next().ok_or(Error::Exhausted)?, MAX_LENGTH)?;
    let num_pieces =
        parse::single_digit(lines.next().ok_or(Error::Exhausted)?, MAX_PIECES)? as usize;

    let mut line_count = 0;
    let mut area_so_far = 0_u64;
    for line in lines.take(num_pieces) {
        let (w, l) = parse::two_digits(line, MAX_LENGTH)?;
        area_so_far += (w * l) as u64;
        line_count += 1;
    }
    if line_count != num_pieces {
        Err(Error::Exhausted)
    } else {
        let cake_length = area_so_far / cake_width as u64;
        writeln!(stdout(), "{}", cake_length)?;
        Ok(())
    }
}
