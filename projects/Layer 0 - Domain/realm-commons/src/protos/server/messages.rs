use teal::net::message::MessageIdentifiable;

use super::server::{BroadcastPlayerListInLobby::BroadcastPlayerListInLobby, CreatedLobby::CreatedLobby, ListLobbies::ListLobbies, RelayInvitationToLobby::RelayInvitationToLobby};
// use super::gen::{BroadcastPlayerListInLobby::BroadcastPlayerListInLobby, CreatedLobby::CreatedLobby, ListLobbies::ListLobbies, RelayInvitationToLobby::RelayInvitationToLobby};

impl MessageIdentifiable for CreatedLobby {
    fn id(&self) -> u16 {
        7101
    }
}

impl MessageIdentifiable for ListLobbies {
    fn id(&self) -> u16 {
        7102
    }
}

impl MessageIdentifiable for RelayInvitationToLobby {
    fn id(&self) -> u16 {
        7103
    }
}

impl MessageIdentifiable for BroadcastPlayerListInLobby {
    fn id(&self) -> u16 {
        7104
    }
}
