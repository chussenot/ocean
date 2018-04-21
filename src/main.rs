#[macro_use]
extern crate clap;
extern crate hyper;
extern crate curl;

use clap::App;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, const_service, service_fn};
use curl::easy::Easy;
use std::time::Duration;
use std::thread::sleep;
use std::process::Command;

static CMD: &'static str = "man vim";

fn main() {
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

    let hello = const_service(service_fn(|_req|{
        Ok(Response::<hyper::Body>::new()
            .with_header(ContentLength(CMD.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(CMD))
    }));

    let server = Http::new().bind(&addr, hello)?;
    server.run()
}
