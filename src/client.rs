use clap::{App, Arg, ArgMatches, SubCommand};

use anyhow::Result;

use hello_world::HelloRequest;
use hello_world::{proxy_client::ProxyClient, HelloReply};

use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("proxy");
}

async fn handle_default(matches: &ArgMatches<'static>) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_add(matches: &ArgMatches<'static>) -> Result<()> {
    println!("Add used! Firing Client Request");
    let mut client = ProxyClient::connect("http://0.0.0.0:50051").await.unwrap();

    let name = matches.value_of("client").unwrap().to_string();

    let request = Request::new(HelloRequest { name });
    match client.say_hello(request).await {
        Ok(v) => println!("Res {:?}", v),
        Err(e) => println!("Err {}", e),
    }
    Ok(())
}

async fn handle_matches(matches: ArgMatches<'static>) -> Result<()> {
    match matches.subcommand() {
        ("add", Some(v)) => handle_add(v).await,
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
            SubCommand::with_name("add").arg(
                Arg::with_name("client")
                    .short("c")
                    .long("client")
                    .value_name("ADDRESS")
                    .help("The address of the eyeball that we will be proxying traffic for")
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
