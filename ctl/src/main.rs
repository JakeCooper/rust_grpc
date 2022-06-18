use clap::{App, Arg, ArgMatches};

use anyhow::Result;

use rust_proxy::proxy_client::ProxyClient;

use rust_proxy::{AddRouteRequest, ListRoutesRequest};

use tonic::transport::Channel;
use tonic::Request;

use crate::rust_proxy::{ListContainersRequest, RemoveRouteRequest, VersionRequest};

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

async fn handle_list(_matches: &ArgMatches) -> Result<()> {
    let mut client = new_client().await?;

    let request = Request::new(ListRoutesRequest {});

    let res = client.list_routes(request).await?.into_inner();

    let r = res
        .routes
        .iter()
        .map(|f| format!("[{}] ({} -> {})", f.uuid, f.from, f.to))
        .collect::<Vec<String>>()
        .join("\n");

    match res.routes.is_empty() {
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

    println!("Route removed {}", uuid);

    Ok(())
}

async fn handle_default(matches: &ArgMatches) -> Result<()> {
    println!("Not implemented! {:?}", matches);
    Ok(())
}

async fn handle_version(_matches: &ArgMatches) -> Result<()> {
    let mut client = new_client().await?;

    let request = Request::new(VersionRequest {});

    let res = client.version(request).await?.into_inner();

    println!(
        "Docker Version:{}",
        serde_json::to_string_pretty(&res.version).unwrap()
    );

    Ok(())
}

async fn handle_container_list(matches: &ArgMatches) -> Result<()> {
    let mut client = new_client().await?;

    let all = matches.is_present("all");

    let request = Request::new(ListContainersRequest { all });

    let res = client.list_containers(request).await?.into_inner();

    let r = res
        .containers
        .iter()
        .map(|f| format!("[{}] ({} -> {})", f.id, f.image, f.status))
        .collect::<Vec<String>>()
        .join("\n");

    match res.containers.is_empty() {
        true => println!("No containers"),
        false => println!("Containers:\n{}", r),
    }

    Ok(())
}

const CONTAINERS_KEY: &str = "containers";
const ROUTES_KEY: &str = "routes";

async fn handle_matches(matches: ArgMatches) -> Result<()> {
    match matches.subcommand().unwrap() {
        (CONTAINERS_KEY, v) => match v.subcommand().unwrap() {
            ("list", s) => handle_container_list(s).await,
            ("version", s) => handle_version(s).await,
            (_, s) => handle_default(s).await,
        },
        (ROUTES_KEY, v) => match v.subcommand().unwrap() {
            ("add", s) => handle_add(s).await,
            ("list", s) => handle_list(s).await,
            ("remove", s) => handle_remove(s).await,
            (_, s) => handle_default(s).await,
        },
        (_, v) => handle_default(v).await,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("proxy_ctl")
        .version("0.1")
        .author("Jake C. <jake@railway.app>")
        .about("a simple, dynamic, proxy with GRPC API")
        // container
        .subcommand(
            App::new(CONTAINERS_KEY)
                .subcommand(
                    App::new("list").arg(
                        Arg::new("all")
                            .short('a')
                            .long("all")
                            .value_name("ALL")
                            .help("Return all containers, not just running ones")
                            .takes_value(false)
                            .required(false),
                    ),
                )
                .subcommand(App::new("version")),
        )
        // route
        .subcommand(
            App::new(ROUTES_KEY)
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
                        Arg::new("uuid")
                            .short('u')
                            .long("uuid")
                            .value_name("UUID")
                            .help("The ID of the route to be removed")
                            .takes_value(true)
                            .required(true),
                    ),
                ),
        )
        .get_matches();

    handle_matches(matches).await
}
