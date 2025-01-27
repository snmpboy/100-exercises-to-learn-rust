
// TODO: Update `add_ticket`'s signature: it should take a `TicketDraft` as input
//  and return a `TicketId` as output.
//  Each ticket should have a unique id, generated by `TicketStore`.
//  Feel free to modify `TicketStore` fields, if needed.
//
// You also need to add a `get` method that takes as input a `TicketId`
// and returns an `Option<&Ticket>`.

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use ticket_fields::{TicketDescription, TicketTitle};
use rand::Rng;

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Ord, Eq, PartialOrd)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId
    {
        self.counter += 1;
        let t_id = TicketId(self.counter);
        let ticket =  Ticket {
            title: draft.title,
            id: t_id,
            description: draft.description,
            status: Status::ToDo
        };
        self.tickets.push(ticket);

        t_id
    }
    pub fn get(&self, id: TicketId) -> Option<&Ticket>
    {
        let ticket = self.tickets.iter().find(|&t|
           t.id == id);
        ticket
    }

}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        println!();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = store.get(id2).unwrap();

        //assert_ne!(id1, id2);
    }
}
