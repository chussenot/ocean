#[macro_use]
extern crate clap;
extern crate hyper;

use clap::App;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response, const_service, service_fn};

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
    loop {
        println!("Loop forever!");
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

