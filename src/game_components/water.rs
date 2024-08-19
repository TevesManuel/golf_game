use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::Shape;
use sfml::graphics::Color;
use sfml::graphics::CircleShape;
use sfml::graphics::Transformable;

use sfml::system::Vector2f;

use crate::basics::window::Window;


pub struct Water
{
    pub position          : Vector2f,
    pub dragged      : bool,
    pub selected     : bool,
    pub radius       : f32,
    pub main_color   : Color,
    pub sec_color    : Color,
}
impl Water
{
    pub fn new(position: Vector2f) -> Water
    {
        Water
        {
            position,
            dragged    : false,
            selected   : false,
            radius     : 10.0,
            // main_color : Color::rgba(174, 255, 247, 255),
            main_color : Color::rgba(0, 197, 255, 220),
            // sec_color  : Color::rgba(250, 203, 121, 255),           
            sec_color  : Color::rgba(250, 203, 121, 255),           
        }
    }
    
    pub fn get_shape(&self) -> CircleShape
    {
        let mut shape = CircleShape::new(self.radius, 30);
        // shape.set_position(self.position - Vector2f::new(self.radius, self.radius));
        shape.set_position(self.position);
        shape.set_origin(Vector2f::new(self.radius, self.radius));
        shape.set_outline_thickness(self.radius*0.1);
        shape.set_fill_color(self.main_color);
        shape.set_outline_color(self.sec_color);
        shape
    }
    
    pub fn is_col(&self, cs: CircleShape) -> bool
    {
        let diff_x = (cs.position().x - self.get_shape().position().x).abs();
        let diff_y = (cs.position().y - self.get_shape().position().y).abs();
        let h      = (diff_x*diff_x + diff_y*diff_y).sqrt();
        let min_distance_to_col = self.radius + cs.radius() + cs.outline_thickness();
        if h <= (min_distance_to_col)
        {
            return true;
        }
        false
    }

    pub fn draw(&self, win : &Window)
    {
        win.window.draw_circle_shape(&self.get_shape(), &RenderStates::DEFAULT);
    }
}