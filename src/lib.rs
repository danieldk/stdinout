//! A pattern that often occurs when writing command-line utilities
//! is that one wants to open a file when a filename argument is
//! provided or read/write from/to stdin/stdout otherwise. Unfortunatlely,
//! this is more work in Rust than it should be.
//!
//! The `stdinout` crate provides a small wrapper that makes it easier
//! to handle this scenario.
//!
//! For reading from a file or the standard input:
//!
//! ```
//! let input = or_stdin(args.get(0));
//! for line in input.unwrap().buf_read().lines() {
//!     // ...
//! }
//! ```
//!
//! For writing to a file or the standard output:
//!
//! ```
//! let output = or_stdout(args.get(1));
//!
//! // Get an object that implements the Write trait.
//! let write = output.write();
//! ```

use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::{Read, BufRead, BufReader, Write};

pub struct InputReader<'a>(Box<BufRead + 'a>);

impl<'a> Read for InputReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<'a> BufRead for InputReader<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}

pub enum Input {
    Stdin(io::Stdin),
    File(PathBuf),
}

impl Input {
    pub fn from<P>(path: Option<P>) -> Self
        where P: Into<PathBuf>
    {
        match path {
            Some(path) => Input::File(path.into()),
            None => Input::Stdin(io::stdin()),
        }
    }

    pub fn buf_read(&self) -> io::Result<InputReader> {
        match self {
            &Input::Stdin(ref stdin) => Result::Ok(InputReader(Box::new(stdin.lock()))),
            &Input::File(ref path) => {
                File::open(path).map(BufReader::new).map(Box::new).map(|r| InputReader(r))
            }
        }
    }
}

pub enum Output {
    Stdout(io::Stdout),
    File(PathBuf),
}

impl Output {
    pub fn from<P>(path: Option<P>) -> Self
        where P: Into<PathBuf>
    {
        match path {
            Some(path) => Output::File(path.into()),
            None => Output::Stdout(io::stdout()),
        }
    }

    pub fn write<'a>(&'a self) -> io::Result<Box<Write + 'a>> {
        match self {
            &Output::Stdout(ref stdout) => Result::Ok(Box::new(stdout.lock())),
            &Output::File(ref path) => Result::Ok(Box::new(try!(File::create(path)))),
        }
    }
}
