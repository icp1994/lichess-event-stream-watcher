#![feature(extern_prelude)]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod conf;
mod event;
mod eventhandler;
mod eventstream;
mod signup;
mod slack;

use futures::future;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    tokio::run(future::lazy(|| {
        let (tx, rx) = channel::<event::Event>();

        tokio::spawn(eventstream::watch_event_stream(
            tx.clone(),
            conf::TOKEN,
            conf::RULES_PATH,
        ));

        eventhandler::handle_events(rx, conf::TOKEN, conf::RULES_PATH);

        Ok(())
    }));
}
