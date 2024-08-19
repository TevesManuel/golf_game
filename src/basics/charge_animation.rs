use sfml::graphics::CircleShape;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderStates;
use sfml::graphics::Shape;
use sfml::graphics::Color;
use sfml::graphics::Transformable;
use sfml::system::Vector2f;

use crate::basics::window::Window;

pub struct ChargeAnimation
{
    pub vel      : f32, 
    pub mode     : bool,
    pub thickness: f32,
    pub radius   : f32,
    pub finished   : bool,
}
impl ChargeAnimation
{
    pub fn new(vel : f32) -> ChargeAnimation
    {
        ChargeAnimation
        {
            vel,
            mode     : false,
            thickness: 0.0,
            radius   : 0.0,
            finished   : false,
        }
    }
    pub fn init(&mut self, mode: bool, window: &Window)
    {
        self.thickness = window.window.size().x as f32;
        if !mode
        {
            self.radius = 0.0;
        }
        else
        {
            self.radius = window.window.size().x as f32;    
        }
        self.mode = mode;
        self.finished = false;
    }
    pub fn update(&mut self, window: &Window)
    {
        // window.window.size().x as f32
        if !self.finished
        {
            if !self.mode
            {
                self.radius += self.vel * window.delta_time;
                if self.radius > window.window.size().x as f32
                {
                    self.radius = window.window.size().x as f32;
                    self.finished = true;
                }
            }
            else 
            {
                self.radius -= self.vel * window.delta_time;
                if self.radius < 0.0
                {
                    self.radius = 0.1; // < 1 pixel = invisible => all black | 0 pixel = nothing => visible background
                    self.finished = true;
                }
            }
        }
        let mut circle = CircleShape::new(self.radius, 40);
        circle.set_fill_color(Color::rgba(0, 0, 0, 0));
        circle.set_origin(Vector2f::new(circle.radius(), circle.radius()));
        circle.set_outline_color(Color::BLACK);
        circle.set_radius(self.radius);
        circle.set_outline_thickness(self.thickness);
        circle.set_position(window.get_middle_point_window());
        window.window.draw_circle_shape(&circle, &RenderStates::DEFAULT);
    }
}