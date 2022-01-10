use clap::{App, Arg, ArgMatches};

use anyhow::Result;

use rust_proxy::proxy_client::ProxyClient;
use rust_proxy::HelloRequest;

use tonic::Request;

pub mod rust_proxy {
    tonic::include_proto!("proxy");
}

async fn handle_hello(matches: &ArgMatches) -> Result<()> {
    println!("Hello used! Firing Client Request");

    let mut client = ProxyClient::connect("http://0.0.0.0:50051").await.unwrap();

    let name = matches.value_of("name").unwrap().to_string();

    let request = Request::new(HelloRequest { name });
    client.say_hello(request).await?;
    Ok(())
}

async fn handle_add(matches: &ArgMatches) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_default(matches: &ArgMatches) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_matches(matches: ArgMatches) -> Result<()> {
    match matches.subcommand().unwrap() {
        ("add", v) => handle_add(v).await,
        ("hello", v) => handle_hello(v).await,
        (_, v) => handle_default(v).await,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("proxy_ctl")
        .version("0.1")
        .author("Jake C. <jake@railway.app>")
        .about("a simple, dynamic, proxy with GRPC API")
        .subcommand(
            App::new("add")
                .arg(
                    Arg::new("from")
                        .short('f')
                        .long("from")
                        .value_name("FROM")
                        .help("The address we're proxying from")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("to")
                        .short('t')
                        .long("to")
                        .value_name("to")
                        .help("The address we're proxying to")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("hello").arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .help("The person you wanna say hello to")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(App::new("list"))
        // .arg(
        //     Arg::with_name("server")
        //         .short("s")
        //         .long("server")
        //         .value_name("ADDRESS")
        //         .help("The address of the origin that we will be proxying traffic for")
        //         .takes_value(true)
        //         .required(true),
        // )
        .get_matches();

    handle_matches(matches).await
}
