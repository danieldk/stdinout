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
//! ```rust,ignore
//! let input = Input::from(matches.free.get(0));
//! let reader = or_exit(input.buf_read());
//!
//! for line in reader.lines() {
//!     // Use 'line'
//! }
//! ```
//!
//! For writing to a file or the standard output:
//!
//! ```rust,ignore
//! let output = Output::from(args.get(1));
//!
//! // Get an object that implements the Write trait.
//! let write = output.write().unwrap();
//! ```

use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::process;

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
    where
        P: Into<PathBuf>,
    {
        match path {
            Some(path) => Input::File(path.into()),
            None => Input::Stdin(io::stdin()),
        }
    }

    pub fn buf_read(&self) -> io::Result<InputReader> {
        match self {
            &Input::Stdin(ref stdin) => Result::Ok(InputReader(Box::new(stdin.lock()))),
            &Input::File(ref path) => File::open(path)
                .map(BufReader::new)
                .map(Box::new)
                .map(|r| InputReader(r)),
        }
    }
}

pub enum Output {
    Stdout(io::Stdout),
    File(PathBuf),
}

impl Output {
    pub fn from<P>(path: Option<P>) -> Self
    where
        P: Into<PathBuf>,
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

macro_rules! stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

/// Types implementing the `OrExit` provide the `or_exit` function that can
/// be used to exit a program when a computation was not successful.
///
/// The goal of this thread is to provide a function similar to `unwrap()`
/// without a panic.
pub trait OrExit<R, S>
where
    S: AsRef<str>,
{
    /// Exit the program with the given message and status code if the
    /// computation is not successful. Otherwise, unwrap the value and
    /// return it.
    fn or_exit(self, message: S, code: i32) -> R;
}

impl<R, S, E> OrExit<R, S> for Result<R, E>
where
    S: AsRef<str>,
    E: fmt::Display,
{
    fn or_exit(self, description: S, code: i32) -> R {
        match self {
            Result::Ok(val) => val,
            Result::Err(err) => {
                stderr!("{}: {}", description.as_ref(), err);
                process::exit(code);
            }
        }
    }
}

impl<R, S> OrExit<R, S> for Option<R>
where
    S: AsRef<str>,
{
    fn or_exit(self, description: S, code: i32) -> R {
        match self {
            Some(val) => val,
            None => {
                stderr!("{}", description.as_ref());
                process::exit(code);
            }
        }
    }
}
