use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use http::{Response, StatusCode};

use crate::ticket::data::{Ticket, TicketDraft, TicketUpdate};
use crate::ticket::store::{self, TicketId};
use serde_json;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::request::{parse_request, Request};

type SharableTicketStore = Arc<RwLock<store::TicketStore>>;
type Sharableticket = Arc<RwLock<Ticket>>;

#[derive(Clone, Copy, Debug)]
pub struct Router {}

impl Router {
    pub async fn dispatch<'header, 'buf>(
        &self,
        req: &Request<'header, 'buf>,
        store: &'static SharableTicketStore,
    ) -> Response<String> {
        match req.request_meta.path {
            Some("/all_tickets/") => {
                let tickets = self.all_tickets(store).await;
                Response::builder()
                    .body(serde_json::to_string(&tickets).unwrap())
                    .unwrap()
            }
            Some("/get_ticket/") => {
                let id: TicketId = serde_json::from_slice(&req.request_body).unwrap();
                let ticket = self.get_ticket(store, id).await;
                match ticket {
                    Some(t) => {
                        let ticket_clone = t.read().await.clone();
                        Response::builder()
                            .body(serde_json::to_string(&ticket_clone).unwrap())
                            .unwrap()
                    }
                    None => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body("Ticket not found".into())
                        .unwrap(),
                }
            }
            Some("/insert_ticket/") => {
                let ticket: TicketDraft = serde_json::from_slice(&req.request_body).unwrap();
                self.insert_ticket(store, ticket).await;
                Response::builder()
                    .status(StatusCode::CREATED)
                    .body("".into())
                    .unwrap()
            }
            Some("/update_ticket/") => {
                let ticket_update: TicketUpdate = serde_json::from_slice(&req.request_body).unwrap();
                match self.get_ticket(store, ticket_update.id).await {
                    Some(ticket) => {
                        let mut ticket_obj = ticket.write().await;
                        if ticket_update.description.is_some() {
                            ticket_obj.description = ticket_update.description.unwrap();
                        };
                        if ticket_update.title.is_some() {
                            ticket_obj.title = ticket_update.title.unwrap();
                        };
                        Response::builder()
                            .body(serde_json::to_string(&*ticket_obj).unwrap())
                            .unwrap()
                    }
                    None => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body("".into())
                        .unwrap(),
                }
            }
            _ => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("".to_string())
                .unwrap(),
        }
    }

    pub async fn get_ticket(
        &self,
        store: &'static SharableTicketStore,
        id: crate::ticket::store::TicketId,
    ) -> Option<Sharableticket> {
        store.read().await.get(id).await
    }

    pub async fn insert_ticket(&self, store: &'static SharableTicketStore, ticket: TicketDraft) {
        {
            store.write().await.add_ticket(ticket).await;
        }
    }

    pub async fn all_tickets(&self, store: &'static SharableTicketStore) -> Vec<TicketId> {
        let tickets = store.read().await;
        tickets.ids().await
    }
}

pub async fn address_connection(
    mut tcp_stream: TcpStream,
    store: &'static SharableTicketStore,
    router: super::router::Router,
) {
    let mut request_buf: Vec<u8> = Vec::with_capacity(1024);
    let mut headers_buf = [httparse::EMPTY_HEADER; 16];
    tcp_stream.read_buf(&mut request_buf).await.unwrap();

    let request: Request = parse_request(&request_buf, &mut headers_buf);

    let resp = router.dispatch(&request, store).await;

    let resp_text = format!(
        "HTTP/1.1 {} Whatever
        Content-Type: text/plain; charset=utf8
        Content-Length: {}
        Content-Type: application/json
        \r\n\r\n{}",
        resp.status(),
        resp.body().len(),
        resp.body()
    );

    tcp_stream.write_all(resp_text.as_bytes()).await.unwrap();
}
