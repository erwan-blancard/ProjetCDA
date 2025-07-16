use crate::server::game::card::{Card, CardId, Element, Kind, Stars, TargetType};
use crate::server::game::play_info::{PlayInfo};

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
    fn get_name(&self) -> String { self.name.clone() }
    fn get_attack(&self) -> u32 { 0 }
    fn get_attack_modifier(&self) -> Option<Box<dyn crate::server::game::modifiers::Modifier>> { None }
    fn get_heal(&self) -> u32 { 0 }
    fn get_heal_modifier(&self) -> Option<Box<dyn crate::server::game::modifiers::Modifier>> { None }
    fn get_draw(&self) -> u32 { 0 }
    fn get_draw_modifier(&self) -> Option<Box<dyn crate::server::game::modifiers::Modifier>> { None }
    fn get_description(&self) -> String { self.desc.clone() }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }
    fn get_target_type(&self) -> TargetType { TargetType::Single }
    fn play(&self, _player_index: usize, _target_indices: Vec<usize>, _game: &mut crate::server::game::game_logic::Game) -> Result<PlayInfo, String> {
        // TODO: logique spéciale
        Ok(PlayInfo::new())
    }
}

#[derive(Debug, Clone)]
pub struct PlayersRollsDiceCard {
    pub id: CardId,
    pub name: String,
    pub element: Element,
    pub stars: Stars,
    pub kind: Kind,
    pub desc: String,
    pub attack: bool,
    pub heal: bool,
    pub draw: bool,
    pub target_type: TargetType,
    pub dice_action: PlayersRollsDiceCardAction,
}

impl Card for PlayersRollsDiceCard {
    fn get_id(&self) -> CardId { self.id }
    fn get_name(&self) -> String { self.name.clone() }
    fn get_attack(&self) -> u32 { 0 }
    fn get_attack_modifier(&self) -> Option<Box<dyn crate::server::game::modifiers::Modifier>> { None }
    fn get_heal(&self) -> u32 { 0 }
    fn get_heal_modifier(&self) -> Option<Box<dyn crate::server::game::modifiers::Modifier>> { None }
    fn get_draw(&self) -> u32 { 0 }
    fn get_draw_modifier(&self) -> Option<Box<dyn crate::server::game::modifiers::Modifier>> { None }
    fn get_description(&self) -> String { self.desc.clone() }
    fn get_kind(&self) -> Kind { self.kind }
    fn get_element(&self) -> Element { self.element }
    fn get_stars(&self) -> Stars { self.stars }
    fn get_target_type(&self) -> TargetType { self.target_type }
    fn play(&self, _player_index: usize, _target_indices: Vec<usize>, _game: &mut crate::server::game::game_logic::Game) -> Result<PlayInfo, String> {
        // TODO: logique spéciale
        Ok(PlayInfo::new())
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "type")]
pub enum PlayersRollsDiceCardAction {
    AffectsMinRollPlayersRollsSum,
    // Ajoute d'autres variantes si besoin
} 