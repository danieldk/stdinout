## Introduction

A pattern that often occurs in UNIX utilities is:

  * You want to read from a file when a filename argument is provided,
    otherwise from stdin.
  * You want to write to a file when a filename argument is provided,
    otherwise to stdout.

This is a small crate that accommodates that pattern.

**Note:** This package is still new, its API will change.

## Installation

This package can be used with Cargo:

    [dependencies]
    stdinout = 0.1
