// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
use http_server::http;
use http_server::ticket::store::TicketStore;

use std::time::Duration;
use tokio::net::TcpListener;

use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref TICKET_STORE: Arc<RwLock<TicketStore>> =
        Arc::new(RwLock::new(TicketStore::new()));
}

pub async fn run(listener: TcpListener, timeout: Duration, router: http::router::Router) {
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let _ = tokio::time::timeout(timeout, async {
            http::router::address_connection(stream, &TICKET_STORE, router).await;
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8787")
        .await
        .expect("Failed to start http server");
    let router = http::router::Router {};
    tokio::spawn(run(listener, Duration::from_secs(10), router))
        .await
        .unwrap();
}
