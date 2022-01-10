use clap::{App, Arg, ArgMatches, SubCommand};

use anyhow::Result;

use rust_proxy::proxy_client::ProxyClient;
use rust_proxy::HelloRequest;

use tonic::Request;

pub mod rust_proxy {
    tonic::include_proto!("proxy");
}

async fn handle_hello(matches: &ArgMatches<'static>) -> Result<()> {
    println!("Hello used! Firing Client Request");

    let mut client = ProxyClient::connect("http://0.0.0.0:50051").await.unwrap();

    let name = matches.value_of("name").unwrap().to_string();

    let request = Request::new(HelloRequest { name });
    client.say_hello(request).await?;
    Ok(())
}

async fn handle_add(matches: &ArgMatches<'static>) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_default(matches: &ArgMatches<'static>) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_matches(matches: ArgMatches<'static>) -> Result<()> {
    match matches.subcommand() {
        ("add", Some(v)) => handle_add(v).await,
        ("hello", Some(v)) => handle_hello(v).await,
        _ => handle_default(&matches).await,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("proxy_ctl")
        .version("0.1")
        .author("Zeke M. <jake@railway.app>")
        .about("a simple, dynamic, proxy with GRPC API")
        .subcommand(
            SubCommand::with_name("add")
                .arg(
                    Arg::with_name("from")
                        .short("f")
                        .long("from")
                        .value_name("FROM")
                        .help("The address we're proxying from")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("to")
                        .short("t")
                        .long("to")
                        .value_name("to")
                        .help("The address we're proxying to")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("hello").arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .value_name("NAME")
                    .help("The person you wanna say hello to")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .subcommand(SubCommand::with_name("list"))
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
