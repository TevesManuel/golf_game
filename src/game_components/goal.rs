use sfml::audio::Sound;
use sfml::graphics::Shape;
use sfml::graphics::Transformable;
use sfml::graphics::CircleShape;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderStates;
use sfml::graphics::Color;
use sfml::system::Vector2f;

use crate::basics::window::Window;

pub struct Goal<'s>
{
    pub position  : Vector2f,
    pub finished  : bool,
    pub radius    : f32,
    pub thickness : f32,
    pub sound_ptr : Sound<'s>,
}
impl Goal<'_>
{
    pub const PERCENT_TO_ENTRY : f32 = 0.85;
    pub const MAX_VEL_TO_ENTRY : f32 = 2000.0;

    pub fn new(sound_ptr : Sound<'_>) -> Goal
    {
        Goal
        {
            position  : Vector2f::new(100.0, 100.0),
            finished  : false,
            radius    : 16.0,
            thickness : 2.0,
            sound_ptr,
        }
    }
    pub fn reset(&mut self)
    {
        self.finished = false;
    }
    pub fn is_col(&self, cs:CircleShape) -> bool
    {
        let diff_x = (cs.position().x - self.get_shape().position().x).abs();
        let diff_y = (cs.position().y - self.get_shape().position().y).abs();
        let h      = (diff_x*diff_x + diff_y*diff_y).sqrt();
        let min_distance_to_col = self.radius + self.thickness + cs.radius() + cs.outline_thickness();
        let sensibility = 0.3;
        if h <= (min_distance_to_col*sensibility)
        {
            return true;
        }
        false
    }
    pub fn check(&self, cs: CircleShape, pv: f32) -> bool
    {
        if pv.abs() <= Goal::MAX_VEL_TO_ENTRY
        {
            return self.is_col(cs.clone())
        }
        false
    }
    pub fn get_shape(&self) -> CircleShape
    {
        let mut shape = CircleShape::new(self.radius, 30);
        // shape.set_position(self.position - Vector2f::new(self.radius, self.radius));
        shape.set_position(self.position);
        shape.set_origin(Vector2f::new(self.radius, self.radius));
        shape.set_outline_thickness(self.thickness);
        shape.set_outline_color(Color::rgba(0,0,0, 100));
        shape.set_fill_color(Color::rgb(30,30,30));
        shape
    }
    pub fn draw(&mut self, win : &Window)
    {
        win.window.draw_circle_shape(&self.get_shape(), &RenderStates::DEFAULT);
    }
}