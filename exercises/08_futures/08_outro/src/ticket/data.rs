use super::store::TicketId;
use super::description::TicketDescription;
use super::title::TicketTitle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct TicketUpdate {
    pub id: TicketId,
    pub title: Option<TicketTitle>,
    pub description: Option<TicketDescription>,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
