extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use clap::{App, Arg, ArgMatches};
use futures::Future;
use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
    let args = App::new("rut")
        .version("0.0")
        .author("Silver <sssilver@gmail.com>")
        .about("Tracks a web resource for availability")
        .arg(Arg::with_name("uri")
             .required(true)
             .help("URI to track")
             .takes_value(true))
        .get_matches();

    run_app(&args);
}


fn run_app(args: &ArgMatches) {
    let mut core = Core::new().unwrap();

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let uri_str = args.value_of("uri").unwrap();
    let uri = uri_str.parse()
        .expect(&format!("URI could not be parsed: {}", uri_str));

    loop {
        let work = client.get(uri).map(|res| {
            println!("Response: {}", res.status());
        });

        core.run(work)
            .expect("no work could be done");
    }
}


