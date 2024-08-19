use std::time::Instant;

use sfml::graphics::RectangleShape;
use sfml::graphics::RenderStates;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::Shape;
use sfml::graphics::Sprite;
use sfml::graphics::Transformable;
use sfml::graphics::Texture;
use sfml::window::mouse;
use sfml::system::Vector2f;
use sfml::system::Vector2u;
use sfml::SfBox;

use crate::level_maker::level_maker::LevelMaker;
impl LevelMaker
{
    fn update_selectable(&mut self, texture: SfBox<Texture>, string_object: &str, pos : Vector2f, size: Vector2f, offset_element: Vector2f)
    {
        let mut rctng_shape = RectangleShape::new();
        rctng_shape.set_size(Vector2f::new(50.0, 50.0));
        rctng_shape.set_position(Vector2f::new(5.0 + rctng_shape.size().x*pos.x + 5.0*pos.x,
        self.subwindow.window.size_title_bar.y+5.0 + rctng_shape.size().y*pos.y + 5.0*pos.y));
        let mut transparency : u8 = 255;
        if self.subwindow.window.get_win_info().mouse_pos.x > rctng_shape.position().x
        && self.subwindow.window.get_win_info().mouse_pos.x < rctng_shape.position().x + rctng_shape.size().x
        && self.subwindow.window.get_win_info().mouse_pos.y < rctng_shape.position().y + rctng_shape.size().y
        && self.subwindow.window.get_win_info().mouse_pos.y > rctng_shape.position().y
        {
            transparency = 200;
        }

        rctng_shape.set_fill_color(Color::rgba(255, 255, 255, transparency));
        self.subwindow.window.window.draw_rectangle_shape(&rctng_shape, &RenderStates::DEFAULT);
        
        let mut sprite = Sprite::new();
        sprite.set_texture(&texture, true);
        sprite.set_position(rctng_shape.position() + offset_element);
        sprite.set_color(Color::rgba(255, 255, 255, transparency));
        sprite.set_scale(size);
        
        self.subwindow.window.window.draw_sprite(&sprite, &RenderStates::DEFAULT);
        if self.subwindow.window.click_left_ant && !mouse::Button::Left.is_pressed()
            && self.subwindow.window.get_win_info().mouse_pos.x > rctng_shape.position().x
            && self.subwindow.window.get_win_info().mouse_pos.x < rctng_shape.position().x + rctng_shape.size().x
            && self.subwindow.window.get_win_info().mouse_pos.y < rctng_shape.position().y + rctng_shape.size().y
            && self.subwindow.window.get_win_info().mouse_pos.y > rctng_shape.position().y
            {
                self.option = Some(String::from(string_object));
            }    
    }
    pub fn subwindow_update(&mut self)
    {
        if !self.on_saving
        {
            self.subwindow.frame();
            let mut rctng_shape = RectangleShape::new();
            rctng_shape.set_size(self.subwindow.window.get_win_info().size);
            rctng_shape.set_fill_color(Color::rgba(30, 30, 30, 255));
            self.subwindow.window.window.draw_rectangle_shape(&rctng_shape, &RenderStates::DEFAULT);

            // println!("{}", &self.water.get_shape().);
            let mut render_texture = sfml::graphics::RenderTexture::new(100, 100).unwrap();
            render_texture.clear(Color::TRANSPARENT);
            render_texture.draw_circle_shape(&self.water.get_shape(), &Default::default());
            // let pointer_to_texture = unsafe { std::ptr::NonNull::new(render_texture.texture() as *const Texture).map(SfBox)};

            self.update_selectable(self.wall.texture.clone(), "WALL", Vector2f::new(0.0,0.0), Vector2f::new(0.75,0.75), Vector2f::new(6.5,6.0));
            self.update_selectable(self.wall.texture.clone(), "WATER", Vector2f::new(1.0,0.0), Vector2f::new(0.75,0.75), Vector2f::new(6.5,6.0));
            // self.update_selectable(, "WATER", Vector2f::new(1.0,0.0), Vector2f::new(0.75,0.75), Vector2f::new(6.5,6.0));
            self.update_selectable(self.wall.texture.clone(), "WALL", Vector2f::new(0.0,1.0), Vector2f::new(0.75,0.75), Vector2f::new(6.5,6.0));
            self.update_selectable(self.wall.texture.clone(), "WALL", Vector2f::new(1.0,1.0), Vector2f::new(0.75,0.75), Vector2f::new(6.5,6.0));

            let vector = self.subwindow.window.window.size() - self.save_button.button.button_pulsed_texture.size();
            self.save_button.button.position = Vector2f::new(vector.x as f32, vector.y as f32);
            self.save_button.button.update(&self.subwindow.window, Instant::now());
            let vector = Vector2u::new(0, self.subwindow.window.window.size().y - self.home_button.button.button_pulsed_texture.size().y);
            self.home_button.button.position = Vector2f::new(vector.x as f32, vector.y as f32);
            self.home_button.button.update(&self.subwindow.window, Instant::now());
            self.subwindow.display();
        }
        self.subwindow.window.window.set_visible(!self.on_saving);
    }
}