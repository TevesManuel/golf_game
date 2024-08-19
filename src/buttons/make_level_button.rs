use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct MakeLevelButton
{
    pub button : Button,
}
impl MakeLevelButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> MakeLevelButton
    {
        let make_level_button: MakeLevelButton = MakeLevelButton
        {
            button : Button::new("./assets/Sprites/MainMenu/MakeLevelPulsed.png", "./assets/Sprites/MainMenu/MakeLevelUnpulsed.png",
                position, movement_effect),
        };

        make_level_button
    }
}