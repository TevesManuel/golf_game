use std::time::Instant;

use sfml::SfBox;
use sfml::graphics::Texture;
use sfml::graphics::IntRect;
use sfml::graphics::Sprite;
use sfml::graphics::RenderStates;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::Transformable;
use sfml::window::mouse;
use sfml::system::Vector2f;

use crate::basics::visual_effects::VisualEffects;
use crate::basics::window::Window;

pub struct Button
{
    pub button_pulsed_texture  : SfBox<Texture>,
    pub button_unpulsed_texture: SfBox<Texture>,
    pub position               : Vector2f      ,
    pub clicked                : bool          ,
    pub clickup                : bool          ,
    pub visual_effect_movement : Vector2f      ,
    pub origin_on_center       : bool          ,
}
impl Button
{
    pub fn new(ppath: &str, upath: &str, position: Vector2f, visual_effect : Vector2f) -> Button
    {
        let texture_pulsed = match Texture::new() 
        {
            Some(x) => x,
            None    => panic!("Error on media creation."),
        };
        let texture_unpulsed = match Texture::new() 
        {
            Some(x) => x,
            None    => panic!("Error on media creation."),
        };

        let mut make_button: Button = Button
        {
            button_pulsed_texture  : texture_pulsed,
            button_unpulsed_texture: texture_unpulsed,
            position,
            clicked                : false,
            clickup                : false,
            visual_effect_movement : visual_effect,
            origin_on_center       : false,
        };

        if make_button.button_pulsed_texture.load_from_file(ppath, IntRect::new(0, 0, 0, 0)).is_err()
        {
            panic!("Error on load texture");
        }
        if make_button.button_unpulsed_texture.load_from_file(upath, IntRect::new(0, 0, 0, 0)).is_err()
        {
            panic!("Error on load texture");
        }

        make_button
    }
    pub fn reset(&mut self)
    {
        self.clicked = false;
        self.clickup = false;
    }
    pub fn get_sprite(&self, passed_time: Instant) -> Sprite
    {
        let mut sprite = Sprite::new();   
        if self.clicked
        {
            sprite.set_texture(&self.button_pulsed_texture, false);
        }
        else
        {
            sprite.set_texture(&self.button_unpulsed_texture, false);            
        }
        if self.origin_on_center
        {
            sprite.set_origin(Vector2f::new(sprite.global_bounds().width/2.0, sprite.global_bounds().height/2.0));
        }
        sprite.set_position(self.position + VisualEffects::calculate_vector_with_time(self.visual_effect_movement, passed_time));

        sprite
    }
    pub fn update(&mut self, win: &Window, passed_time: Instant)
    {
        self.clickup = false;
        if win.window.has_focus()
        {
            if self.clicked && !mouse::Button::Left.is_pressed()
            {
                self.clickup = true;
                self.clicked = false;
            }
            if self.is_over(win.get_win_info().mouse_pos) && win.click_left_down
            {
                self.clicked = true;
            }
        }
        let mut sprite = self.get_sprite(passed_time);
        if self.is_over(win.get_win_info().mouse_pos)
        {
            sprite.set_color(Color::rgb(200, 200, 200));
        }
        win.window.draw_sprite(&sprite, &RenderStates::DEFAULT);
    }
    fn is_over(&self, mouse_pos: Vector2f) -> bool
    {
        let bounds = self.get_sprite(Instant::now()).global_bounds();

        if mouse_pos.x > bounds.left
        && mouse_pos.x < bounds.left + bounds.width
        && mouse_pos.y > bounds.top
        && mouse_pos.y < bounds.top + bounds.height
        {
            return true;
        }
        
        false
    }
}