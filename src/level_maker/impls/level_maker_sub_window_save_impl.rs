use sfml::graphics::RenderTarget;
use sfml::graphics::Text;
use sfml::graphics::Transformable;
use sfml::window::Key;
use sfml::graphics::RectangleShape;
use sfml::graphics::RenderStates;
use sfml::graphics::Color;
use sfml::graphics::Shape;
use sfml::system::Vector2f;

use crate::level_maker::level_maker::LevelMaker;
use crate::basics::window::Window;
use crate::global_properties::GlobalProperties;

impl LevelMaker
{
    fn save_frame(&mut self, win : &mut Window)
    {
        let mut background = RectangleShape::new();
        background.set_size(win.get_win_info().size);
        background.set_fill_color(Color::rgba(0,0,0,200));
        let mut semi_background = RectangleShape::new();
        semi_background.set_size(win.get_win_info().size/2.0);
        semi_background.set_origin((win.get_win_info().size/2.0)/2.0);
        semi_background.set_position(win.get_middle_point_window());
        semi_background.set_outline_thickness(10.0);
        semi_background.set_fill_color(GlobalProperties::BACKGROUND_COLOR_SECUNDARY_WINDOW);
        semi_background.set_outline_color(Color::BLACK);
        
        //AUTHOR INPUT
        self.input_author.size = Vector2f::new(semi_background.size().x*0.8, 
                                                     semi_background.size().y*0.1);
        self.input_author.pos  = Vector2f::new(semi_background.position().x,
                                           semi_background.position().y*0.7);
        let mut author_text = Text::new("Your name", &self.font, 10);
        author_text.set_position(self.input_author.pos - Vector2f::new(0.0,20.0) - self.input_author.size/2.0);
        author_text.set_fill_color(Color::BLACK);

        //PROJECT INPUT
        self.input_project.size = Vector2f::new(semi_background.size().x*0.8, 
        semi_background.size().y*0.1);
        self.input_project.pos  = Vector2f::new(semi_background.position().x,
            semi_background.position().y*0.95);
        let mut project_text = Text::new("Level name", &self.font, 10);
        project_text.set_position(self.input_project.pos - Vector2f::new(0.0,20.0) - self.input_project.size/2.0);
        project_text.set_fill_color(Color::BLACK);

        win.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);
        win.window.draw_rectangle_shape(&semi_background, &RenderStates::DEFAULT);
        self.input_author.frame(win, self.font.clone());
        self.input_project.frame(win, self.font.clone());
        win.window.draw_text(&author_text, &RenderStates::DEFAULT);        
        win.window.draw_text(&project_text, &RenderStates::DEFAULT);        
    }
    pub fn subwindow_save_update(&mut self, win : &mut Window)
    {
        if self.on_saving
        {
            self.save_frame(win);
            if Key::Enter.is_pressed()
            {
                self.save_level();
            }
        }
    }
}