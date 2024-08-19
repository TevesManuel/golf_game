use sfml::system::Vector2f;
use sfml::system::Vector2i;
use sfml::graphics::Font;
use sfml::SfBox;

use crate::basics::window::Window;

pub struct SubWindow
{
    pub window: Window,
    pub pos   : Vector2i,
    pub font  : SfBox<Font>,
    pub title : String,
    pub size  : Vector2f,
}
impl SubWindow
{
    pub fn new(font: SfBox<Font>, title: String, size: Vector2f) -> SubWindow
    {
        let mut sub_win = Window::new(title.clone(), size, font.clone());
        sub_win.enabled_close_button = false;
        sub_win.window.set_visible(false);
        sub_win.window.set_framerate_limit(60);
        SubWindow
        {
            window : sub_win,
            pos    : Vector2i::new(-1, -1),
            font,
            size,
            title,
        }
    }
    pub fn reset(&mut self)
    {
        if self.window.window.is_open()
        {
            self.window.window.close();
        }
        let mut sub_win = Window::new(self.title.clone(), self.size, self.font.clone());
        sub_win.enabled_close_button = false;
        sub_win.window.set_visible(false);
        sub_win.window.set_framerate_limit(60);
        if self.pos.x != -1 && self.pos.y != -1
        {
            sub_win.window.set_position(self.pos);
        }
        self.window = sub_win;
    }
    pub fn frame(&mut self)
    {
        self.window.frame();
    }
    pub fn display(&mut self)
    {
        self.window.display();
    }
    pub fn set_position(&mut self, pos: Vector2i)
    {
        self.pos = pos;
        self.window.window.set_position(self.pos);
    }
}