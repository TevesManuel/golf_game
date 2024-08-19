use sfml::system::Vector2f;

use crate::basics::button::Button;

pub struct ArrowButton
{
    pub button : Button,
}
impl ArrowButton
{
    pub fn new(position: Vector2f, movement_effect: Vector2f, orientation: bool) -> ArrowButton
    {
        let arrow_button: ArrowButton = match orientation
        {
            true =>
            {
                ArrowButton
                {
                    button : Button::new("./assets/Sprites/ArrowUp_P.png", "./assets/Sprites/ArrowUp_U.png",
                        position, movement_effect),
                }
            },
            false =>
            {
                ArrowButton
                {
                    button : Button::new("./assets/Sprites/ArrowDown_P.png", "./assets/Sprites/ArrowDown_U.png",
                        position, movement_effect),
                }
            }
        };

        arrow_button
    }
}