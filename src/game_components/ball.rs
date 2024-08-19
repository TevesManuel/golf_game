use sfml::graphics::CircleShape;
use sfml::graphics::Shape;
use sfml::graphics::Color;
use sfml::graphics::Transformable;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget; 
use sfml::system::Vector2f;

use crate::basics::window::Window;
use crate::basics::win_info::WinInfo;
use crate::basics::physics::Physics;

use std::time::Instant;

pub struct Ball
{
    pub shape                : CircleShape<'static>,
    pub physics              : Physics,
    pub posible_new_shape    : CircleShape<'static>,
    pub from_color           : Color,
    pub color                : Option<Color>,
    pub color_thickness      : Option<Color>,
    pub on_goal              : Option<Instant>,
    pub finished_goal_anim   : bool,
    pub last_position        : Vector2f,
}
impl Ball
{
    pub fn new(middle_screen_pos: Vector2f, color_ball: Color) -> Ball
    {
        let radius            : f32      = 10.0;
        let thickness         : f32      = 5.0;
        let mut ball = Ball
        {
            shape               : CircleShape::new(radius, 30),
            physics             : Physics::new(),
            posible_new_shape   : CircleShape::new(radius, 30),
            from_color          : color_ball,
            color               : None,
            color_thickness     : None,
            on_goal             : None,
            finished_goal_anim  : false,
            last_position       : Vector2f::new(0.0, 0.0),
        };
        ball.shape.set_outline_color(Color::BLACK);
        ball.shape.set_outline_thickness(thickness);
        ball.shape.set_origin(Vector2f::new(ball.shape.radius(), ball.shape.radius()));
        ball.shape.set_position(middle_screen_pos);
        ball.posible_new_shape = ball.shape.clone();
        ball
    }
    pub fn reset(&mut self)
    {
        self.physics = Physics::new();
        self.on_goal = None;
        self.finished_goal_anim = false;
        self.shape.set_fill_color(self.from_color);
    }
    fn calculate_pos_is_out_screen(&mut self, position: Vector2f, wi: WinInfo) -> bool
    {
        if position.x - self.shape.radius() - self.shape.outline_thickness() < 0.0 || position.x + self.shape.radius() + self.shape.outline_thickness() > wi.size.x
        {
            self.physics.force.x *= -1.0;
            return false;
        }
        if position.y + self.shape.radius() + self.shape.outline_thickness() > wi.size.y || position.y - self.shape.radius() - self.shape.outline_thickness() < wi.size_title_bar.y
        {
            self.physics.force.y *= -1.0;
            return false;
        }
        true
    }
    pub fn update(&mut self, wi: WinInfo)
    {
        if (self.physics.force.x != 0.0 || self.physics.force.y != 0.0) && self.on_goal.is_none()
        {
            let vel: Vector2f = Vector2f::new(self.physics.force.x/self.physics.mass, self.physics.force.y/self.physics.mass);
            let posible_new_position: Vector2f = Vector2f::new(
                self.shape.position().x + vel.x*wi.delta_time,
                self.shape.position().y + vel.y*wi.delta_time
            );
            if self.calculate_pos_is_out_screen(posible_new_position, wi.clone())
            {
                self.posible_new_shape.set_position(posible_new_position);
            }
            self.physics.force.x -= self.physics.force.x * self.physics.friction * wi.delta_time;
            self.physics.force.y -= self.physics.force.y * self.physics.friction * wi.delta_time;
        }
    }
    pub fn application_of_position(&mut self)
    {
        self.shape = self.posible_new_shape.clone();
    }
    pub fn draw(&mut self, win : &Window)
    {
        if self.color.is_some()
        {
            self.shape.set_fill_color(self.color.unwrap());
        }
        if self.color_thickness.is_some()
        {
            self.shape.set_outline_color(self.color_thickness.unwrap());
        }
        
        if self.on_goal.is_some() && !self.finished_goal_anim
        {
            let duration = 1000.0;
            let step_time = duration/255.0;
            let mut c = self.shape.fill_color();
            let opacidad = 255 - (self.on_goal.unwrap().elapsed().as_millis() as f32/step_time) as u8;
            c.a = opacidad;
            self.shape.set_fill_color(c);             
            let mut c = self.shape.outline_color();
            c.a = opacidad;
            self.shape.set_outline_color(c);
            if c.a == 0
            {
                self.finished_goal_anim = true;
            }
        }
        if self.finished_goal_anim
        {
            let mut c = self.shape.fill_color();
            c.a = 0;
            self.shape.set_fill_color(c);
            let mut c = self.shape.outline_color();
            c.a = 0;
            self.shape.set_outline_color(c);
        }

        win.window.draw_circle_shape(&self.shape.clone(), &RenderStates::DEFAULT);
    }
    pub fn is_over(&mut self, x_pos : Vector2f) -> bool
    {
        if x_pos.x >= self.shape.position().x - self.shape.radius()
        && x_pos.x <= self.shape.position().x + self.shape.radius()
        && x_pos.y >= self.shape.position().y - self.shape.radius()
        && x_pos.y <= self.shape.position().y + self.shape.radius()
        {
            return true;
        }
        false
    }
}