use std::f32::consts::PI;

use sfml::graphics::RectangleShape;
use sfml::graphics::Transformable;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderStates;
use sfml::system::Vector2f;

use crate::basics::window::Window;

pub struct TrajectoryViewer
{
    pub shape  : RectangleShape<'static>,
    pub angle  : f32,
}
impl TrajectoryViewer
{
    pub fn new() -> TrajectoryViewer
    {
        TrajectoryViewer
        {
            shape: RectangleShape::new(),
            angle: 0.0,
        }
    }
    pub fn update(&mut self, from_position: Vector2f, to_position: Vector2f)
    {
        self.calculate_angle(from_position, to_position);
        self.shape = RectangleShape::new();
        self.shape.set_position(from_position);

        let diff_x     : f32 = from_position.x - to_position.x;
        let diff_y     : f32 = from_position.y - to_position.y;
        let radiovector: f32 = (diff_x*diff_x + diff_y*diff_y).sqrt();
        
        self.shape.set_size(Vector2f::new(radiovector, 5.0));
        self.shape.set_rotation(self.angle);
    }
    fn calculate_angle(&mut self, from_position: Vector2f, to_position: Vector2f)
    {
        let diff_x     : f32 = from_position.x - to_position.x;
        let diff_y     : f32 = from_position.y - to_position.y;
        let radiovector: f32 = (diff_x*diff_x + diff_y*diff_y).sqrt();
        let angle      : f32 = (diff_x/radiovector).asin()  * 180.0/PI;
        let offset     : f32;
        if to_position.y > from_position.y
        {
            offset = 90.0;
            self.angle = offset + angle;
        }
        else
        {
            offset = 270.0;
            self.angle = offset - angle
        }
    }
    pub fn draw(&self, win: &Window)
    {
        win.window.draw_rectangle_shape(&self.shape.clone(), &RenderStates::DEFAULT);
    }
}