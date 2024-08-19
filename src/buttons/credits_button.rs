use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct CreditsButton
{
    pub button : Button,
}
impl CreditsButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> CreditsButton
    {
        let credits_button: CreditsButton = CreditsButton
        {
            button : Button::new("./assets/Sprites/MenuPause/CButtonPulsed.png", "./assets/Sprites/MenuPause/CButtonUnpulsed.png",
                position, movement_effect),
        };

        credits_button
    }
}