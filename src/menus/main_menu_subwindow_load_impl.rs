use sfml::graphics::RectangleShape;
use sfml::graphics::Color;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::Shape;
use sfml::graphics::Text;
use sfml::graphics::Transformable;
use sfml::system::Vector2f;

use crate::basics::window::Window;
use crate::global_properties::GlobalProperties;

use super::main_menu::MainMenu;

impl MainMenu
{
    pub fn level_loader_sub_window_update(&mut self)
    {
        let mut background = RectangleShape::new();
        background.set_fill_color(Color::rgb(0, 0, 0));
        background.set_size(self.level_loader_sub_window.window.get_win_info().size);
        self.level_loader_sub_window.window.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);
        self.arrow_button_up.button.update(&self.level_loader_sub_window.window, self.init_time);
        self.arrow_button_down.button.position.y = self.level_loader_sub_window.window.get_win_info().size.y - self.arrow_button_down.button.button_pulsed_texture.size().y as f32;
        self.arrow_button_down.button.update(&self.level_loader_sub_window.window, self.init_time);
        let mut background = RectangleShape::new();
        background.set_position(Vector2f::new(0.0, self.arrow_button_up.button.button_pulsed_texture.size().y as f32 + self.arrow_button_up.button.position.y));
        background.set_fill_color(GlobalProperties::BACKGROUND_COLOR_SECUNDARY_WINDOW);
        background.set_size(Vector2f::new(self.level_loader_sub_window.window.get_win_info().size.x, self.level_loader_sub_window.window.get_win_info().size.y - self.arrow_button_down.button.button_pulsed_texture.size().y as f32*2.0 - Window::get_title_bar_size().y));
        self.level_loader_sub_window.window.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);
        if (self.arrow_button_up.button.clickup 
        || self.level_loader_sub_window.window.delta_wheel_mouse > 0)
        && self.levels_index != 0
        {
            self.levels_index -= 1;
        }
        if (self.arrow_button_down.button.clickup
        || self.level_loader_sub_window.window.delta_wheel_mouse < 0)
        && self.levels_index != self.levels.len()-2
        {
            self.levels_index += 1;
        }
        for i in 0..2
        {
            let index = self.levels_index + i;
            if index >= self.levels.len()
            {
                break;
            }
            let mut string_out = self.levels[index].project_name.clone();
            string_out        += " - ";
            string_out        += self.levels[index].author_name.as_str();

            let mut text = Text::new(string_out.as_str(), &self.font, 12);
            text.set_fill_color(Color::WHITE);
            let offset = Vector2f::new(20.0,20.0);
            text.set_position(Vector2f::new(0.0,i as f32*50.0 + Window::get_title_bar_size().y + self.arrow_button_up.button.button_pulsed_texture.size().y as f32));
            text.set_position(text.position() + offset);
            text.set_outline_color(Color::BLACK);
            text.set_outline_thickness(2.0);

            let mut text_background = RectangleShape::new();
            text_background.set_position(Vector2f::new(0.0, text.position().y - offset.y*0.6));
            text_background.set_size(Vector2f::new(self.level_loader_sub_window.window.get_win_info().size.x, text.global_bounds().size().y + offset.y));
            let mut nc = background.fill_color();
            if self.level_loader_sub_window.window.get_win_info().mouse_pos.x > text_background.position().x && self.level_loader_sub_window.window.get_win_info().mouse_pos.y < text_background.position().x + text_background.size().x
            && self.level_loader_sub_window.window.get_win_info().mouse_pos.y > text_background.position().y && self.level_loader_sub_window.window.get_win_info().mouse_pos.y < text_background.position().y + text_background.size().y
            {
                nc.a = 128;
                text.set_outline_color(Color::BLUE);
                if self.level_loader_sub_window.window.click_left_ant && !sfml::window::mouse::Button::Left.is_pressed()
                {
                    self.level_path = Some(self.levels[index].path.clone());
                    self.finished   = true;
                    self.option     = 1;
                    self.level_loader_sub_window.window.window.close();
                }
            }
            else
            {
                nc.a = 0;
            }
            nc.r = (nc.r as f32 * 0.5) as u8;
            nc.g = (nc.g as f32 * 0.5) as u8;
            nc.b = (nc.b as f32 * 0.5) as u8;
            text_background.set_fill_color(nc);

            self.level_loader_sub_window.window.window.draw_rectangle_shape(&text_background, &RenderStates::DEFAULT);
            self.level_loader_sub_window.window.window.draw_text(&text, &RenderStates::DEFAULT);
        }
    }
}