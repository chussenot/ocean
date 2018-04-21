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

static TEXT: &'static str = "Hello, Ocean!";

fn main() {
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();
    if let Some(_matches) = _matches.subcommand_matches("serve") {
        let _result = serve();
    } else if let Some(_matches) = _matches.subcommand_matches("connect") {
        connect();
    }
}

fn connect() {
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
           .expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        sleep(Duration::from_millis(1000));
    }
}

fn serve() -> Result<(), hyper::Error> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let hello = const_service(service_fn(|_req|{
        Ok(Response::<hyper::Body>::new()
            .with_header(ContentLength(TEXT.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(TEXT))
    }));

    let server = Http::new().bind(&addr, hello)?;
    server.run()
}
