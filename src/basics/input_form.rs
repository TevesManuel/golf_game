use sfml::SfBox;
use sfml::graphics::Font;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::Text;
use sfml::graphics::Transformable;
use sfml::system::Vector2f;
use sfml::graphics::RectangleShape;
use sfml::graphics::Color;
use sfml::graphics::Shape;
use sfml::window::Key;
use sfml::window::mouse;
use crate::basics::window::Window;
use crate::global_properties::GlobalProperties;

pub struct InputForm
{
    pub size      : Vector2f,
    pub pos       : Vector2f,
    pub text      : String,
    pub focused   : bool,
    pub font_size : u32,
}
impl InputForm
{
    pub fn new() -> InputForm
    {
        InputForm
        {
            size      : Vector2f::new(0.0,0.0),
            pos       : Vector2f::new(0.0,0.0),
            text      : "".to_owned(),
            focused   : false,
            font_size : 10,
        }
    }
    pub fn frame(&mut self, win : &mut Window, font: SfBox<Font>)
    {
        let mut background = RectangleShape::new();
        background.set_size(self.size);
        background.set_origin(self.size/2.0);
        background.set_position(self.pos);
        background.set_outline_thickness(5.0);
        background.set_outline_color(Color::BLACK);
        if win.get_win_info().mouse_pos.x > self.pos.x - self.size.x/2.0
        && win.get_win_info().mouse_pos.x < self.pos.x + self.size.x/2.0
        && win.get_win_info().mouse_pos.y > self.pos.y - self.size.y/2.0
        && win.get_win_info().mouse_pos.y < self.pos.y + self.size.y/2.0
        {
            background.set_fill_color(Color::rgb(200, 200, 200));
            if mouse::Button::Left.is_pressed()
            {
                self.focused = true;
            }
        }
        else if mouse::Button::Left.is_pressed()
        {
            self.focused = false;
        }
        if self.focused && win.key_up_char.is_some()
        {
            if (win.key_up_char.unwrap().is_alphabetic() || win.key_up_char.unwrap().is_alphanumeric()    
            ||  win.key_up_char.unwrap() == '_' || win.key_up_char.unwrap() == ' ')
            && self.text.len() < GlobalProperties::MAX_LENGHT_NAME
            {
                self.text.push(win.key_up_char.unwrap());
            }
            if win.key_up.unwrap() == Key::Backspace && !self.text.is_empty()
            {
                self.text.remove(self.text.len()-1);
            }
        }

        let mut text = Text::new(&self.text, &font, self.font_size);
        text.set_position(self.pos - Vector2f::new(self.size.x/2.2, self.size.y*0.15));
        text.set_outline_thickness(1.0);
        text.set_outline_color(Color::BLACK);
        win.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);
        win.window.draw_text(&text, &RenderStates::DEFAULT);
    }
}