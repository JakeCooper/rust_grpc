use clap::{App, Arg, ArgMatches};

use anyhow::Result;

use rust_proxy::proxy_client::ProxyClient;

use rust_proxy::{AddRouteRequest, ListRoutesRequest, Route};

use tonic::transport::Channel;
use tonic::Request;

use crate::rust_proxy::RemoveRouteRequest;

pub mod rust_proxy {
    tonic::include_proto!("proxy");
}

async fn new_client() -> Result<ProxyClient<Channel>> {
    Ok(ProxyClient::connect("http://0.0.0.0:50051").await?)
}

async fn handle_add(matches: &ArgMatches) -> Result<()> {
    let mut client = new_client().await?;

    let from = matches.value_of("from").unwrap().to_string();
    let to = matches.value_of("to").unwrap().to_string();

    let request = Request::new(AddRouteRequest { from, to });

    client.add_route(request).await?;
    Ok(())
}

async fn handle_list(matches: &ArgMatches) -> Result<()> {
    let mut client = new_client().await?;

    let request = Request::new(ListRoutesRequest {});

    let res = client.list_routes(request).await?.into_inner();

    let r = res
        .route
        .iter()
        .map(|f| format!("({} -> {})", f.from, f.to))
        .collect::<Vec<String>>()
        .join("\n");

    match res.route.is_empty() {
        true => println!("No routes"),
        false => println!("Routes:\n{}", r),
    }

    Ok(())
}

async fn handle_remove(matches: &ArgMatches) -> Result<()> {
    let mut client = new_client().await?;

    let uuid = &matches.value_of("uuid").unwrap().to_string();

    let request = Request::new(RemoveRouteRequest {
        uuid: uuid.to_string(),
    });

    client.remove_route(request).await?;

    println!("Route removed {}", uuid.to_string());

    Ok(())
}

async fn handle_default(matches: &ArgMatches) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_matches(matches: ArgMatches) -> Result<()> {
    match matches.subcommand().unwrap() {
        ("add", v) => handle_add(v).await,
        ("list", v) => handle_list(v).await,
        ("remove", v) => handle_remove(v).await,
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
        .subcommand(App::new("list"))
        .subcommand(
            App::new("remove").arg(
                Arg::new("from")
                    .short('f')
                    .long("from")
                    .value_name("FROM")
                    .help("The address we're proxying from")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .get_matches();

    handle_matches(matches).await
}
