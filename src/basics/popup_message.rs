use std::time::Instant;

use sfml::graphics::RectangleShape;
use sfml::graphics::Shape;
use sfml::graphics::Color;
use sfml::graphics::Transformable;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderStates;
use sfml::graphics::Font;
use sfml::graphics::Text;
use sfml::system::Vector2f;
use sfml::SfBox;

use crate::basics::window::Window;

pub struct PopupMessage
{
    pub string              : String,
    pub transparency        : f32,
    pub state               : u8,// 0 -> up 1 -> down
    pub size                : Vector2f,
    pub vel                 : f32,
    pub lecture_i           : Option<Instant>,
    pub duration            : f32,
    pub limit_transparency  : f32,
    pub font                : SfBox<Font>,
    pub charsize            : u32,
    pub offset_text         : Vector2f,
    // pub   : f32,
}
impl PopupMessage
{
    pub fn new(font : SfBox<Font>) -> PopupMessage
    {
        PopupMessage
        {
            string             : "saved".to_owned(),
            transparency       : 0.0,
            state              : 3,//NONE 
            size               : Vector2f::new(200.0,40.0),
            vel                : 2.0,
            lecture_i          : None,
            duration           : 1.0,
            limit_transparency : 0.8,
            font,
            charsize           : 15,
            offset_text        : Vector2f::new(0.0,0.0),
        }
    }
    pub fn init(&mut self, string : String)
    {
        self.state = 0;
        self.string = string;
    }
    pub fn update(&mut self, win: &Window)
    {
        if self.state == 2
        {
            return ;
        }
        if self.transparency >= 1.0 && self.state == 0 && self.lecture_i.is_none()
        {
            self.lecture_i = Some(Instant::now());
        }
        if self.transparency >= 1.0 && self.lecture_i.is_some()
        && self.lecture_i.unwrap().elapsed().as_secs_f32() > self.duration
        {
            self.state = 1;
        }
        if self.transparency < 0.0 && self.state == 1
        {
            self.state = 2;
        }
        
        /*
        --------------------------
             Funcion Clamp
        --------------------------
        float_number.clamp(min, max);
        Literalmente hace lo que el codigo if de abajo, mantiene el minimo en el minimo y vicebersa
        */
        // if self.transparency > 1.0 //Remplaced by clamp
        // {
        //     self.transparency = 1.0;
        // }
        // if self.transparency < 0.0
        // {
        //     self.transparency = 0.0;
        // }
        self.transparency = self.transparency.clamp(0.0, 1.0);
        if self.state == 0
        {
            self.transparency += self.vel * win.delta_time;
        }
        if self.state == 1
        {
            self.transparency -= self.vel * win.delta_time;
        }
        let mut rectangle_container = RectangleShape::new();
        rectangle_container.set_fill_color(Color::rgba(30,30,30,(255.0*self.transparency) as u8));
        rectangle_container.set_size(self.size);
        rectangle_container.set_origin(Vector2f::new(self.size.x/2.0, self.size.y/2.0));
        rectangle_container.set_position(win.get_middle_point_window());
        win.window.draw_rectangle_shape(&rectangle_container, &RenderStates::DEFAULT);
        let mut text = Text::new(&self.string, &self.font, self.charsize);
        text.set_fill_color(Color::rgba(255, 255, 255, (255.0*self.transparency) as u8));
        text.set_position(rectangle_container.position() - text.global_bounds().size()/2.0);
        win.window.draw_text(&text, &RenderStates::DEFAULT);
    }
}