use crate::gameplay::game_manager::GameManager;

pub trait GameRendererSender {
    fn game_sender(&self, game: &GameManager);
}
