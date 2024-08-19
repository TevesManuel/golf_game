use sfml::graphics::CircleShape;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::Shape;
use sfml::graphics::Transformable;
use sfml::graphics::RenderStates;
use sfml::graphics::RectangleShape;
use sfml::system::Vector2f;
use crate::basics::window::Window;

pub struct Slider
{
    pub size      : Vector2f,
    pub pos       : Vector2f,
    pub dragged   : bool,
    pub input_pos : Vector2f,
    pub max       : f32,
    pub min       : f32,
}
impl Slider
{
    pub fn new() -> Slider
    {
        Slider
        {
            size      : Vector2f::new(0.0, 0.0),
            pos       : Vector2f::new(0.0, 0.0),
            dragged   : false,
            input_pos : Vector2f::new(0.0, 0.0),
            max       : 0.0,
            min       : 0.0,
        }
    }
    pub fn set_input(&mut self, input : f32)
    {
        // println!("{}", (input-self.min)/(self.max-self.min));
        self.input_pos.x = ((input-self.min)/(self.max-self.min) * self.size.x) + (self.pos.x - self.size.x/2.0)
    }
    pub fn get_input(&mut self) -> f32
    {
        // println!("{}", (((self.pos.x-(self.size.x/2.0)) - self.input_pos.x).abs() * 100.0) / self.size.x);
        let percentage = ((self.pos.x-(self.size.x/2.0))-self.input_pos.x).abs() / self.size.x;
        // println!("{}", (percentage*(GlobalProperties::MAX_WALL_SCALE-GlobalProperties::MIN_WALL_SCALE)) + GlobalProperties::MIN_WALL_SCALE);
        (percentage*(self.max-self.min)) + self.min
    }
    pub fn frame(&mut self, win : &mut Window)
    {
        let mut slide_background = RectangleShape::new();
        slide_background.set_size(self.size);
        slide_background.set_origin(slide_background.size()/2.0);
        slide_background.set_position(self.pos);
        slide_background.set_fill_color(Color::BLACK);
        let mut slide_ball = CircleShape::new(10.0, 10);
        slide_ball.set_outline_thickness(2.0);
        slide_ball.set_outline_color(Color::BLACK);
        slide_ball.set_origin(slide_ball.global_bounds().size()/2.0);
        let pos = self.input_pos + Vector2f::new(0.0, slide_background.position().y);
        slide_ball.set_position(pos);
        if win.get_win_info().mouse_pos.x > slide_ball.global_bounds().left
        && win.get_win_info().mouse_pos.x < slide_ball.global_bounds().left + slide_ball.global_bounds().width
        && win.get_win_info().mouse_pos.y > slide_ball.global_bounds().top
        && win.get_win_info().mouse_pos.y < slide_ball.global_bounds().top + slide_ball.global_bounds().height
        {
            slide_ball.set_fill_color(Color::rgb(200, 200, 200));
            if sfml::window::mouse::Button::Left.is_pressed()
            {
                self.dragged = true;
            }
        }
        if !sfml::window::mouse::Button::Left.is_pressed()
        {
            self.dragged = false;
        }
        if self.dragged
        {
            self.input_pos.x = win.get_win_info().mouse_pos.x;
            self.input_pos.x = self.input_pos.x.clamp(slide_background.global_bounds().left, slide_background.global_bounds().left + slide_background.global_bounds().width);
        }
        win.window.draw_rectangle_shape(&slide_background, &RenderStates::DEFAULT);
        win.window.draw_circle_shape(&slide_ball, &RenderStates::DEFAULT);
    }
}