use std::{error::Error, sync::Arc};

use async_trait::async_trait;
use coral_commons::{
    protos::models::{MatchResult, MatchState},
    red::red_match,
};
use prost_reflect::DynamicMessage;
use realm_commons::red::red_player;
use teal::{
    net::{client::DefaultClient, handler::MessageHandler},
    DynamicClient,
};

#[derive(Debug, Default)]
pub(crate) struct MatchResultHandler;
#[async_trait]
impl MessageHandler for MatchResultHandler {
    async fn handle(
        &self,
        msg: DynamicMessage,
        client: &DynamicClient,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = msg.transcode_to::<MatchResult>().unwrap();
        println!("hey coral got {:?}", message);

        let player_id = client.get_id_ref().lock().await.clone();

        unsafe {
            if let Some(db) = &mut crate::DB {
                let owner = red_player::get(db, &player_id)?;
                let mut game = red_match::get(db, &owner.game)?;

                // Update game
                game.state = MatchState::Finished as i32;
                red_match::set(db, &game).ok();

                // Update players
                let players = &game.players;
                let owner_team = players.get(&owner.id).unwrap();
                for (p, team) in &game.players {
                    let mut player = red_player::get(db, p)?;
                    // Reset game
                    player.game = "".to_string();
                    // Add recent match
                    player.recent_matches.push(game.id.clone());
                    // Add elo
                    if owner_team.eq(team) {
                        player.mmr += 50;
                    } else {
                        player.mmr -= 50;
                    }
                    // Update
                    red_player::set(db, &player).ok();
                }
            }
        }

        Ok(())
    }
}
