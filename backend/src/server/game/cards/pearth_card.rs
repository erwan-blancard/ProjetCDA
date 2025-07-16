use std::collections::HashSet;

use super::card::{Card, CardId, Element, Kind, Stars, TargetType};
use super::super::game::Game;
use super::super::play_info::{PlayAction, PlayInfo};

/// Card variant for Pearth card
#[derive(Debug, Clone)]
pub struct PearthCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
}

impl Card for PearthCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { String::from(&self.name) }
    fn get_description(&self) -> String { String::from(&self.desc) }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }
    fn get_target_type(&self) -> TargetType { TargetType::All }

    fn play(&self, player_index: usize, _target_indices: Vec<usize>, game: &mut Game) -> Result<(PlayInfo, HashSet<usize>), String> {
        let mut info: PlayInfo = PlayInfo::new();

        let dicards = game.players.iter().map(|player| player.discard_cards.len()).sum::<usize>();

        let mut action: PlayAction = PlayAction::new();
        let action_target = game.players[player_index].heal(dicards as u32, self.get_heal_effect());
        action.targets.push(action_target);

        info.actions.push(action);

        Ok((info, HashSet::new()))
    }
}
