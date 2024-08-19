use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct HomeButton
{
    pub button : Button,
}
impl HomeButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> HomeButton
    {
        let make_level_button: HomeButton = HomeButton
        {
            button : Button::new("./assets/Sprites/MenuPause/MButtonPulsed.png", "./assets/Sprites/MenuPause/MButtonUnpulsed.png",
                position, movement_effect),
        };

        make_level_button
    }
}