use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct ReplayButton
{
    pub button : Button,
}
impl ReplayButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> ReplayButton
    {
        ReplayButton
        {   
            button : Button::new("./assets/Sprites/MenuPause/ButtonReplayPulsed.png", "./assets/Sprites/MenuPause/ButtonReplayUnpulsed.png",
            position, movement_effect),
        }
    }
}