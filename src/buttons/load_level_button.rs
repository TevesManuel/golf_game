use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct LoadLevelButton
{
    pub button : Button,
}
impl LoadLevelButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> LoadLevelButton
    {
        let make_level_button: LoadLevelButton = LoadLevelButton
        {
            button : Button::new("./assets/Sprites/MainMenu/LoadLevelPulsed.png", "./assets/Sprites/MainMenu/LoadLevelUnpulsed.png",
                position, movement_effect),
        };
        
        make_level_button
    }
}