//! A convenient wrapper for the [`c-ares`](http://c-ares.haxx.se) library.
//!
//! The [`c-ares` crate](https://crates.io/crates/c-ares) provides a safe
//! wrapper around the underlying C library, but it's relatively hard work to
//! use: the user needs to drive the polling of file descriptors according to
//! `c-ares` demands, which likely involves writing something close to a
//! full-blown event loop.
//!
//! This crate does that hard work for you so that the presented API is much
//! more straightforward.  Simply create a `Resolver`, and make your query -
//! providing a callback to be called when the query completes.
//!
//! This crate also provides a `FutureResolver`.  Queries on this object don't
//! take callbacks, and return `futures::Future` objects.
//!
//! (The API on the `FutureResolver` isn't _quite_ as complete as on the
//! `Resolver`.  For some types of query, the values returned by `c-ares` do
//! not have a long enough lifetime to be returned in a `Future`.  The relevant
//! functions are not supported by the `FutureResolver`.)
//!
//! Complete examples showing how to use the library can be found
//! [here](https://github.com/dimbleby/c-ares-resolver/tree/master/examples).
extern crate c_ares;
extern crate futures;
extern crate mio;

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate ws2_32;

mod error;
mod eventloop;
mod futureresolver;
mod resolver;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub use error::Error;
pub use futureresolver::FutureResolver;
pub use resolver::{
    Options,
    Resolver,
};
