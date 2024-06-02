use std::collections::BTreeMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::data::{Status, Ticket, TicketDraft};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TicketId{
    id: u64
}

#[derive(Clone, Debug)]
pub struct TicketStore {
    tickets: Arc<RwLock<BTreeMap<TicketId, Arc<RwLock<Ticket>>>>>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Arc::new(RwLock::new(BTreeMap::new())),
            counter: 0,
        }
    }

    pub async fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId{id: self.counter};
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        {
            let mut tickets = self.tickets.write().await;
            tickets.insert(id, ticket);
        }
        id
    }

    pub async fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets
           .read()
           .await
           .get(&id)
           .cloned()
    }

    pub async fn ids(&self) -> Vec<TicketId> {
        self.tickets
           .read()
           .await
           .keys()
           .cloned()
           .collect::<Vec<_>>()
    }
}


impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}