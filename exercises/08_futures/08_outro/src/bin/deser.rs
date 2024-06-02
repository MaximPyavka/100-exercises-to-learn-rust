use http_server::ticket::store::TicketId;

fn main() {
    let data = r#" { "id": 23 }"#;

    let ticket: TicketId = serde_json::from_slice(data.as_bytes()).unwrap();

    println!("{:?}", ticket);
}