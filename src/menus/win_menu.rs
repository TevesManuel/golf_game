use std::time::Instant;

use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::RectangleShape;
use sfml::graphics::Shape;
use sfml::graphics::RenderStates;
use sfml::graphics::Transformable;
use sfml::graphics::Font;
use sfml::graphics::Text;

use sfml::SfBox;

use sfml::system::Vector2f;

use crate::buttons::home_button::HomeButton;
use crate::buttons::replay_button::ReplayButton;

use crate::basics::visual_effects::VisualEffects;
use crate::basics::window::Window;

use crate::global_properties::GlobalProperties;

pub struct WinMenu
{
    pub title_position_text             : Vector2f,
    pub play_button                     : ReplayButton,
    pub home_button                     : HomeButton,
    pub font                            : SfBox<Font>,
    pub init_time                       : Instant,
    pub title_movement_multiplier       : Vector2f,
}
impl WinMenu
{
    pub fn new(font: SfBox<Font>, win : &Window) -> WinMenu
    {
        let buttons_movement = Vector2f::new(0.0, -2.0);
        
        WinMenu
        {
            title_position_text             : Vector2f::new(0.0, -100.0),
            title_movement_multiplier       : Vector2f::new(1.0, 1.0),
            play_button                     : ReplayButton::new(win.get_middle_point_window() + Vector2f::new(0.0, 30.0), buttons_movement),
            home_button                     : HomeButton::new(win.get_middle_point_window() + Vector2f::new(0.0, 90.0), buttons_movement),
            font,          
            init_time                       : Instant::now(),
        }
    }
    pub fn reset(&mut self)
    {
        self.home_button.button.reset();
        self.play_button.button.reset();
    }
    /*
        out of update
        2 -> replay
        1 -> back to home   
    */
    pub fn update(&mut self, win : &Window, hits: u8) -> Option<u8>
    {
        if self.play_button.button.clickup
        {
            return Some(2);
        }
        if self.home_button.button.clickup
        {
            return Some(1);
        }
        
        let mut background = RectangleShape::new();
        background.set_fill_color(GlobalProperties::BACKGROUND_SEMITRANSPARENT);
        background.set_size(win.get_win_info().size);
        win.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);

        let mut background = RectangleShape::new();
        background.set_fill_color(GlobalProperties::BACKGROUND_COLOR_SECUNDARY_WINDOW);
        background.set_size(win.get_win_info().size / 2.0);
        background.set_origin(background.size()/2.0);
        background.set_position(win.get_middle_point_window());
        win.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);

        let mut text_title = Text::new("WIN", &self.font, 38);
        text_title.set_origin(Vector2f::new(text_title.global_bounds().width/2.0,
                                            text_title.global_bounds().height/2.0));
        text_title.set_position(win.get_middle_point_window() + self.title_position_text + VisualEffects::calculate_vector_with_time(self.title_movement_multiplier, self.init_time));
        text_title.set_fill_color(Color::YELLOW);
        text_title.set_outline_thickness(5.0);
        text_title.set_outline_color(Color::BLACK);
        win.window.draw_text(&text_title, &RenderStates::DEFAULT);
        
        let mut text_title : String = "hits: ".to_owned();
        text_title += &hits.to_string();
        let mut text_title = Text::new(&text_title, &self.font, 12);
        text_title.set_origin(Vector2f::new(text_title.global_bounds().width/2.0,
                                            text_title.global_bounds().height/2.0));
        text_title.set_position(win.get_middle_point_window() + Vector2f::new(0.0, 60.0) + self.title_position_text + VisualEffects::calculate_vector_with_time(self.title_movement_multiplier, self.init_time));
        text_title.set_fill_color(Color::YELLOW);
        text_title.set_outline_thickness(5.0);
        text_title.set_outline_color(Color::BLACK);
        win.window.draw_text(&text_title, &RenderStates::DEFAULT);

        self.play_button.button.origin_on_center = true;
        self.home_button.button.origin_on_center = true;
        self.play_button.button.update(win, self.init_time);
        self.home_button.button.update(win, self.init_time);
        
        None
    }
}