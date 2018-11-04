use std::{process, io::{self, stdin, stdout, BufRead, BufWriter}};

#[derive(Debug)]
enum Error {
    Exhausted,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

mod parse {}

fn main() -> Result<(), Error> {
    let (stdin, stdout) = (stdin(), stdout());
    let (mut stdin_lock, stdout_lock) = (stdin.lock(), stdout.lock());
    let mut writer = BufWriter::new(stdout_lock);

    let mut line_buf = String::new();
    loop {
        line_buf.clear();
        stdin_lock.read_line(&mut line_buf)?;

        if line_buf.len() == 0 {
            return Err(Error::Exhausted);
        }
    }
}
