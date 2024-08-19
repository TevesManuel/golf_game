use sfml::system::Vector2f;
use sfml::graphics::Font;
use sfml::SfBox;

use crate::basics::window::Window;

pub struct LevelLoaderSubWindow
{
    pub window: Window,
    pub font  : SfBox<Font>,
}
impl LevelLoaderSubWindow
{
    const WINSIZE : Vector2f = Vector2f::new(400.0, 200.0);

    pub fn new(font: SfBox<Font>) -> LevelLoaderSubWindow
    {
        let mut win = Window::new("LEVEL LOADER".to_owned(), LevelLoaderSubWindow::WINSIZE, font.clone());
        win.window.set_visible(false);
        win.insta_close = true;
        LevelLoaderSubWindow
        {
            window : win,
            font,
        }
    }
    pub fn frame(&mut self)
    {
        self.window.frame();
    }
    pub fn display(&mut self)
    {
        self.window.display();
    }
    pub fn reset(&mut self)
    {
        self.window = Window::new("LEVEL LOADER".to_owned(), LevelLoaderSubWindow::WINSIZE, self.font.clone());
        self.window.window.set_visible(false);
        self.window.insta_close = true;
    }
}