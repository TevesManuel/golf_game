use std::process::Command;
use std::time::Instant;

use sfml::SfBox;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::RectangleShape;
use sfml::graphics::Shape;
use sfml::graphics::RenderStates;
use sfml::graphics::Transformable;
use sfml::graphics::Font;
use sfml::graphics::Text;
use sfml::system::Vector2f;
use sfml::window::Key;

use crate::buttons::credits_button::CreditsButton;
use crate::buttons::home_button::HomeButton;
use crate::buttons::play_button::PlayButton;

use crate::basics::visual_effects::VisualEffects;
use crate::basics::window::Window;

pub struct PauseMenu
{
    pub pause_background_color          : Color,
    pub is_paused                       : bool,
    pub title_position_text             : Vector2f,
    pub play_button                     : PlayButton,
    pub credits_button                  : CreditsButton,
    pub home_button                     : HomeButton,
    pub key_p_state_ant                 : bool,
    pub key_esc_state_ant                 : bool,
    pub font                            : SfBox<Font>,
    pub init_time                       : Instant,
    pub title_movement_multiplier       : Vector2f,
}
impl PauseMenu
{
    pub fn new(font: SfBox<Font>) -> PauseMenu
    {
        let buttons_movement = Vector2f::new(0.0, -2.0);
        
        PauseMenu
        {
            pause_background_color          : Color::rgba(30, 30, 30, 200),
            is_paused                       : false,
            title_position_text             : Vector2f::new(45.0, 200.0),
            play_button                     : PlayButton::new(Vector2f::new(145.0,300.0), buttons_movement),
            credits_button                  : CreditsButton::new(Vector2f::new(145.0,385.0), buttons_movement),
            home_button                     : HomeButton::new(Vector2f::new(145.0, 470.0), buttons_movement),
            key_p_state_ant                 : false,
            key_esc_state_ant               : false,
            font,          
            init_time                       : Instant::now(),
            title_movement_multiplier       : Vector2f::new(10.0, 10.0),
        }
    }

    pub fn set_pause(&mut self, state : bool)
    {
        self.is_paused = state;
        self.init_time = Instant::now();
    }

    pub fn update(&mut self, win : &Window)
    {
        if (self.key_p_state_ant   && !Key::P.is_pressed())
        || (self.key_esc_state_ant && !Key::Escape.is_pressed())
        {
            self.is_paused = !self.is_paused;
        }
        self.key_p_state_ant = Key::P.is_pressed();
        self.key_esc_state_ant = Key::Escape.is_pressed();
        if self.is_paused
        {
            if self.play_button.button.clickup
            {
                self.set_pause(false);
            }
            if self.credits_button.button.clickup
            {
                if cfg!(target_os = "windows")
                {
                    Command::new("cmd")
                            .args(["/C", "start https://www.instagram.com/teves_manu/"])
                            .output()
                            .expect("failed to execute process");
                }
                self.credits_button.button.clicked = false;
            }
            
            let mut background = RectangleShape::new();
            background.set_fill_color(self.pause_background_color);
            background.set_size(win.get_win_info().size);
            win.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);

            let mut text_title = Text::new("GolfGame", &self.font, 38);
            text_title.set_position(self.title_position_text + VisualEffects::calculate_vector_with_time(self.title_movement_multiplier, self.init_time));
            text_title.set_fill_color(Color::WHITE);
            text_title.set_outline_thickness(5.0);
            text_title.set_outline_color(Color::BLACK);
            win.window.draw_text(&text_title, &RenderStates::DEFAULT);

            self.play_button.button.update(win, self.init_time);
            self.credits_button.button.update(win, self.init_time);
            self.home_button.button.update(win, self.init_time);
        }
    }
}