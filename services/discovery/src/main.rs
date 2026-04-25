mod arp;
mod observation;
mod probe;
mod sink;

use std::time::Duration;

use clap::Parser;
use ipnet::Ipv4Net;
use tracing_subscriber::EnvFilter;

use crate::observation::{default_ports, observation_from_probe};
use crate::probe::probe_subnet;
use crate::sink::{HttpSink, ObservationSink, StdoutSink};

#[derive(Parser, Debug)]
#[command(name = "discovery", about = "Active subnet discovery for the semantic registry")]
struct Args {
    /// CIDR subnet to probe, e.g. 192.168.1.0/24
    subnet: Ipv4Net,

    /// Comma-separated TCP ports. Defaults to a small OT/IT mix.
    #[arg(long, value_delimiter = ',')]
    ports: Option<Vec<u16>>,

    /// Maximum concurrent host probes.
    #[arg(long, default_value_t = 64)]
    concurrency: usize,

    /// Per-port TCP connect timeout in milliseconds.
    #[arg(long, default_value_t = 500)]
    timeout_ms: u64,

    /// Semantic-DNS base URL. If unset, observations print to stdout.
    #[arg(long, env = "SEMANTIC_DNS_URL")]
    semantic_dns: Option<String>,

    /// Bearer token for the semantic-DNS sink.
    #[arg(long, env = "SEMANTIC_DNS_TOKEN")]
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let args = Args::parse();
    let ports = args.ports.unwrap_or_else(default_ports);

    let sink: Box<dyn ObservationSink> = match args.semantic_dns {
        Some(url) => {
            tracing::info!(url = %url, "posting observations to semantic-dns");
            Box::new(HttpSink::new(url, args.token))
        }
        None => {
            tracing::info!("no SEMANTIC_DNS_URL set, writing observations to stdout");
            Box::new(StdoutSink)
        }
    };

    tracing::info!(
        subnet = %args.subnet,
        ports = ?ports,
        concurrency = args.concurrency,
        "starting probe"
    );

    let results = probe_subnet(
        args.subnet,
        ports,
        args.concurrency,
        Duration::from_millis(args.timeout_ms),
    )
    .await;

    tracing::info!(hosts = results.len(), "probe finished");

    // Scrape AFTER the probe so the kernel's ARP cache has fresh entries from
    // our own TCP handshakes.
    let arp_table = arp::scrape().await;
    tracing::info!(arp_entries = arp_table.len(), "arp scrape complete");

    let mut sent = 0u64;
    let mut failed = 0u64;
    for result in &results {
        let mac = arp_table.get(&result.ip).map(String::as_str);
        let observation = observation_from_probe(result, mac);
        match sink.send(&observation).await {
            Ok(()) => sent += 1,
            Err(err) => {
                failed += 1;
                tracing::warn!(ip = %result.ip, error = %err, "sink rejected observation");
            }
        }
    }

    tracing::info!(sent, failed, "done");
}
