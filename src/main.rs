mod parse {
    use std::io;

    #[derive(Debug)]
    pub enum Error {
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
    pub fn digit_stop_at(
        input: &[u8],
        max_value: u32,
        stop_byte: u8,
    ) -> Result<(u32, &[u8]), Error> {
        let mut digits = [0; 7];
        let mut num_digits = 0;
        for d in input
            .iter()
            .take_while(|&&b| b != stop_byte)
            .map(|b| b - b'0')
        {
            digits[num_digits] = d;
            num_digits += 1;
        }

        let res = digits[..num_digits]
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, &b10)| {
                acc + DIGIT_POWER[index] * b10 as u32
            });

        if res > max_value {
            Err(Error::InvalidDomain(res, max_value))
        } else {
            Ok((res, &input[num_digits + 1..]))
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
    let (cake_width, cursor) = parse::digit_stop_at(&buf, MAX_LENGTH, b'\n')?;
    let (num_pieces, mut cursor) = parse::digit_stop_at(&cursor, MAX_PIECES, b'\n')?;
    let num_pieces = num_pieces as usize;

    let mut area_so_far = 0_u64;
    for _ in 0..num_pieces {
        let (w, ncursor) = parse::digit_stop_at(cursor, MAX_LENGTH, b' ')?;
        let (l, ncursor) = parse::digit_stop_at(ncursor, MAX_LENGTH, b'\n')?;
        cursor = ncursor;
        area_so_far += (w * l) as u64;
    }

    let cake_length = area_so_far / cake_width as u64;
    writeln!(stdout(), "{}", cake_length)?;
    Ok(())
}
