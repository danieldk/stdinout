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
use std::path::{Path, PathBuf};
use std::io;
use std::io::{BufRead, BufReader, Write};

pub enum Input {
    Stdin(io::Stdin),
    File(PathBuf),
}

impl Input {
    pub fn from(filename: Option<&str>) -> Self
    {
        match filename {
            Some(n) => Input::File(Path::new(n.into()).to_owned()),
            None => Input::Stdin(io::stdin()),
        }
    }

    pub fn buf_read<'a>(&'a self) -> io::Result<Box<BufRead + 'a>> {
        match self {
            &Input::Stdin(ref stdin) => Result::Ok(Box::new(stdin.lock())),
            &Input::File(ref path) => Result::Ok(Box::new(BufReader::new(try!(File::open(path))))),
        }
    }
}

pub enum Output {
    Stdout(io::Stdout),
    File(PathBuf),
}

impl Output {
    pub fn from(filename: Option<&str>) -> Self
    {
        match filename {
            Some(n) => Output::File(Path::new(n.into()).to_owned()),
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