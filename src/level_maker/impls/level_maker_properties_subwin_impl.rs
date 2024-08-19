use sfml::graphics::RectangleShape;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::Shape;
use sfml::graphics::Text;
use sfml::graphics::Transformable;
use sfml::system::Vector2f;

use crate::level_maker::level_maker::LevelMaker;
use crate::global_properties::GlobalProperties;

impl LevelMaker
{
    fn sc_prop_frame(&mut self)//size & color
    {
        let mut background = RectangleShape::new();
        background.set_size(self.subwindow_prop.size);
        background.set_fill_color(GlobalProperties::BACKGROUND_COLOR_SECUNDARY_WINDOW);
        self.size_slider.size = Vector2f::new(self.subwindow_prop.size.x*0.8, 10.0);
        self.size_slider.pos  = Vector2f::new(self.subwindow_prop.size.x*0.5, self.subwindow_prop.size.y*0.25);
        self.size_slider.min  = GlobalProperties::MIN_WALL_SCALE;
        self.size_slider.max  = GlobalProperties::MAX_WALL_SCALE;
        let mut text_scale = Text::new("SCALE", &self.font, 10);
        text_scale.set_position(self.size_slider.pos - self.size_slider.size/2.0 - Vector2f::new(0.0, 15.0));
        self.subwindow_prop.window.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);
        self.size_slider.frame(&mut self.subwindow_prop.window);
        self.subwindow_prop.window.window.draw_text(&text_scale, &RenderStates::DEFAULT);
    }
    pub fn subwindow_properties_update(&mut self)
    {
        //Manager for properties sub window
        if self.is_selected_any_wall()
        {
            if self.is_selected_any_wall()
            {
                for wall in self.walls.iter_mut()
                {
                    if wall.selected  
                    {
                        if wall.scale.is_some()
                        {
                            self.size_slider.set_input(wall.scale.unwrap().x);
                        }
                        else
                        {
                            self.size_slider.set_input(1.0);    
                        }
                    }
                }
            }
            self.subwindow_prop.window.window.set_visible(true);
            self.subwindow_prop.frame();
            self.sc_prop_frame();
            self.subwindow_prop.display();
            if self.is_selected_any_wall()
            {
                for wall in self.walls.iter_mut()
                {
                    if wall.selected
                    {
                        wall.scale = None;
                        if self.size_slider.get_input() > 0.0
                        {
                            wall.scale = Some(Vector2f::new(self.size_slider.get_input(), self.size_slider.get_input()));
                        }
                    }
                }
            }
        }
        else
        {
            self.subwindow_prop.window.window.set_visible(false);
            self.offset_on_selection = None;
        }
    }
}