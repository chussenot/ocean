#[macro_use]
extern crate clap;
extern crate hyper;
extern crate futures;
extern crate curl;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate tokio_core;

use clap::App;
use hyper::header::{ContentLength};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

use futures::future::Future;
use curl::easy::Easy;
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;
use std::io;

struct OceanService;
static CMD: &'static str = "man vim";

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
        client();
    }
}

fn client() {
    let mut handle = Easy::new();
    loop {
        println!("Loop forever!");
        handle.url("http://127.0.0.1:3000").unwrap();
        handle.perform().unwrap();
        println!("{}", handle.response_code().unwrap());
        let output = Command::new("sh")
           .arg("-c")
           .arg("man ed")
           .output()
           .expect("Failed to execute process");
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
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
        // We're currently ignoring the Request
        // And returning an 'ok' Future, which means it's ready
        // immediately, and build a Response with the 'PHRASE' body.

        let mut response = Response::new();

         match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let stdin = io::stdin();
                let input = &mut String::new();

                println!("{}", "shell>");
                input.clear();
                stdin.read_line(input);
                println!("{}", input);
                response.set_body("ls");
            },
            (&Method::Post, "/") => {
                // we'll be back
                println!("{}", "Yes!");
                response.set_body("Thx for the fish!");
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}
