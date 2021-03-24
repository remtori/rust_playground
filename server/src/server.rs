use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use actix::prelude::*;
use bytes::Bytes;
use log::info;

/// Server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub enum Message {
    Text(String),
    Binary(Bytes),
}

/// New session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub bytes: Bytes,
}

pub struct GameServer {
    sessions: HashMap<usize, Recipient<Message>>,
    visitor_count: Arc<AtomicUsize>,
}

impl GameServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> GameServer {
        GameServer {
            sessions: HashMap::new(),
            visitor_count,
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for GameServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let id = self.visitor_count.fetch_add(1, Ordering::SeqCst) + 1;
        self.sessions.insert(id, msg.addr);

        info!("User {} joined", id);

        id
    }
}

impl Handler<Disconnect> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        info!("User {} disconnected", msg.id);

        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        for (id, session) in self.sessions.iter() {
            if *id != msg.id {
                let _ = session.do_send(Message::Binary(msg.bytes.clone()));
            }
        }
    }
}
