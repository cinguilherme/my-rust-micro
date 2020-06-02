use std::fmt;
use std::sync::{Arc, Mutex};
use slab::Slab;
use regex::Regex;
use futures::{future, Future};
use hyper::service::service_fn;
use hyper::{Server, Body, Request, Response, Error, Method};

fn main() {
    println!("Hello, world!");
}
