use sfml::SfBox;
use sfml::graphics::Texture;
use sfml::graphics::IntRect;
use sfml::graphics::Sprite;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderStates;
use sfml::system::Vector2i;

use crate::basics::window::Window;

pub struct Background
{
    pub background_texture: SfBox<Texture>,
    pub offset            : Vector2i,
}
impl Background
{
    pub fn new(offset: Vector2i) -> Background
    {
        let texture = match Texture::new() 
        {
            Some(x) => x,
            None    => panic!("Error on background creation."),
        };

        let mut background: Background = Background
        {
            background_texture: texture,
            offset,
        };

        if background.background_texture.load_from_file("./assets/Sprites/Game/background.png", IntRect::new(0, 0, 0, 0)).is_err()
        {
            panic!("Error on load texture");
        }
        background.background_texture.set_repeated(true);

        background
    }
    pub fn draw(&self, win : &Window)
    {
        let mut sprite: Sprite = Sprite::new();
        sprite.set_texture(&self.background_texture, false);
        sprite.set_texture_rect(IntRect::new(self.offset.x, self.offset.y, win.window.size().x as i32, win.window.size().y as i32));
        win.window.draw_sprite(&sprite, &RenderStates::DEFAULT);
    }
    pub fn clone(&self) -> Background
    {
        Background
        {
            background_texture: self.background_texture.clone(),
            offset: self.offset,
        }
    }
}