use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use ipnet::Ipv4Net;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub ip: Ipv4Addr,
    pub open_ports: Vec<u16>,
    pub hostname: Option<String>,
}

pub async fn probe_subnet(
    subnet: Ipv4Net,
    ports: Vec<u16>,
    host_concurrency: usize,
    per_port_timeout: Duration,
) -> Vec<ProbeResult> {
    let sem = Arc::new(Semaphore::new(host_concurrency));
    let ports = Arc::new(ports);
    let mut handles = Vec::new();

    for ip in subnet.hosts() {
        let sem = sem.clone();
        let ports = ports.clone();
        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire_owned().await.ok()?;
            probe_host(ip, &ports, per_port_timeout).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(Some(result)) = handle.await {
            results.push(result);
        }
    }
    results.sort_by_key(|r| u32::from(r.ip));
    results
}

async fn probe_host(
    ip: Ipv4Addr,
    ports: &[u16],
    per_port_timeout: Duration,
) -> Option<ProbeResult> {
    let mut open_ports = Vec::new();
    for &port in ports {
        let addr = SocketAddr::new(IpAddr::V4(ip), port);
        if let Ok(Ok(_stream)) = timeout(per_port_timeout, TcpStream::connect(addr)).await {
            open_ports.push(port);
        }
    }

    if open_ports.is_empty() {
        return None;
    }

    let hostname = reverse_dns(ip).await;
    Some(ProbeResult { ip, open_ports, hostname })
}

async fn reverse_dns(ip: Ipv4Addr) -> Option<String> {
    let ip_str = ip.to_string();
    tokio::task::spawn_blocking(move || dns_lookup::lookup_addr(&IpAddr::V4(ip)).ok())
        .await
        .ok()
        .flatten()
        .filter(|name| name != &ip_str)
}
