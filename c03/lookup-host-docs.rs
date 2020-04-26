#![feature(lookup_host)]

use std::net;

fn main() {
    for host in net::lookup_host("rust-lang.org")? {
        println!("found address: {}", host);
    }
}
