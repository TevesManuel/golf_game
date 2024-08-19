use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct SaveButton
{
    pub button : Button,
}
impl SaveButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f) -> SaveButton
    {
        SaveButton
        {
            button : Button::new("./assets/Sprites/LVL MAKER/PSAVE.png", "./assets/Sprites/LVL MAKER/USAVE.png",
        position, movement_effect),
        }
    }
}