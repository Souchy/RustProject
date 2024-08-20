use teal::net::message::MessageIdentifiable;

use crate::protos::{
    client::{CreateLobby, Identify, JoinLobby, QuitLobby, SendInvitationToLobby},
    models::{Lobby, Player, User},
    server::{BroadcastPlayerListInLobby, CreatedLobby, ListLobbies, RelayInvitationToLobby},
};

const REALM: u16 = 9000;
const MODEL: u16 = REALM + 0;
const REQUEST: u16 = REALM + 100;
const RESPONSE: u16 = REALM + 200;

// Models
impl MessageIdentifiable for User {
    fn id(&self) -> u16 {
        MODEL + 1
    }
}
impl MessageIdentifiable for Player {
    fn id(&self) -> u16 {
        MODEL + 2
    }
}
impl MessageIdentifiable for Lobby {
    fn id(&self) -> u16 {
        MODEL + 3
    }
}
// Client
impl MessageIdentifiable for Identify {
    fn id(&self) -> u16 {
        REQUEST + 1
    }
}
impl MessageIdentifiable for CreateLobby {
    fn id(&self) -> u16 {
        REQUEST + 2
    }
}
impl MessageIdentifiable for JoinLobby {
    fn id(&self) -> u16 {
        REQUEST + 3
    }
}
impl MessageIdentifiable for SendInvitationToLobby {
    fn id(&self) -> u16 {
        REQUEST + 4
    }
}
impl MessageIdentifiable for QuitLobby {
    fn id(&self) -> u16 {
        REQUEST + 5
    }
}
// Server
impl MessageIdentifiable for CreatedLobby {
    fn id(&self) -> u16 {
        RESPONSE + 1
    }
}
impl MessageIdentifiable for ListLobbies {
    fn id(&self) -> u16 {
        REQUEST + 2
    }
}
impl MessageIdentifiable for RelayInvitationToLobby {
    fn id(&self) -> u16 {
        REQUEST + 3
    }
}
impl MessageIdentifiable for BroadcastPlayerListInLobby {
    fn id(&self) -> u16 {
        REQUEST + 4
    }
}
