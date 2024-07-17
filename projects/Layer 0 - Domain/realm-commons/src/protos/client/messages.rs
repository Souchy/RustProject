use teal::net::message::MessageIdentifiable;

use super::client::{CreateLobby::CreateLobby, JoinLobby::JoinLobby, SendInvitationToLobby::SendInvitationToLobby};

// use super::gen::{CreateLobby::CreateLobby, JoinLobby::JoinLobby, SendInvitationToLobby::SendInvitationToLobby};

impl MessageIdentifiable for CreateLobby {
    fn id(&self) -> u16 {
        7001
    }
}

impl MessageIdentifiable for JoinLobby {
    fn id(&self) -> u16 {
        7002
    }
}

impl MessageIdentifiable for SendInvitationToLobby {
    fn id(&self) -> u16 {
        7003
    }
}

