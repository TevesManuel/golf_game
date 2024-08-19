use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct PlayButton
{
    pub button : Button,
}
impl PlayButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> PlayButton
    {
        PlayButton
        {
            button : Button::new("./assets/Sprites/MenuPause/ButtonPulsed.png", "./assets/Sprites/MenuPause/ButtonUnpulsed.png",
            position, movement_effect),
        }
    }
}