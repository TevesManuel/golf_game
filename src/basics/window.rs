use sfml::SfBox;
use sfml::graphics::RenderWindow;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::RectangleShape;
use sfml::graphics::Transformable;
use sfml::graphics::RenderStates;
use sfml::graphics::Shape;
use sfml::graphics::Font;
use sfml::graphics::Text;
use sfml::system::Vector2f;
use sfml::system::Vector2i;
use sfml::window::Style;
use sfml::window::Event;
use sfml::window::Key;
use sfml::window::mouse;

use std::time::Instant;

use crate::basics::win_info::WinInfo;

pub struct Window
{ 
    pub window               : RenderWindow,
    pub delta_time           : f32,
    pub init_frame_time      : Instant,
    pub size_title_bar       : Vector2f,
    pub font                 : SfBox<Font>,
    pub title                : String,
    pub click_left_ant       : bool,
    pub click_left_up        : bool,
    pub click_left_down      : bool,
    pub is_dragged           : bool,
    pub offset_draw_mouse    : Vector2i,
    pub title_text_position  : Vector2f,
    pub close_button_clicked : bool,
    pub on_closing           : bool,
    pub enabled_close_button : bool,
    pub insta_close          : bool,
    pub key_up_char          : Option<char>,
    pub key_up               : Option<Key>,
    pub delta_wheel_mouse    : i8,//Negative for down and posititive for up
}

impl Window
{
    pub fn get_title_bar_size() -> Vector2f
    {
        Vector2f::new(0.0, 20.0)
    }

    pub fn new(title: String, size: Vector2f, font: SfBox<Font>) -> Window
    {
        let size_title_bar = Window::get_title_bar_size();
        let size = size + size_title_bar;
        let title_text_position    : Vector2f = Vector2f::new(5.0, 3.0);
        let mut win = Window
        {
            window               : RenderWindow::new((size.x as u32, size.y as u32), &title, Style::NONE, &Default::default()),
            delta_time           : 0.0,
            init_frame_time      : Instant::now(),
            size_title_bar,
            font,
            title,
            click_left_ant       : false,
            click_left_up        : false,
            click_left_down        : false,
            is_dragged           : false,
            offset_draw_mouse    : Vector2i::new(0, 0),
            title_text_position,
            close_button_clicked : false,
            on_closing           : false,
            enabled_close_button : true,
            insta_close          : false,
            key_up_char          : None,
            key_up               : None,
            delta_wheel_mouse    : 0,
        };
        win.window.set_framerate_limit(60);
        win
    }
    pub fn get_win_info(&self) -> WinInfo
    {
        WinInfo
        {
            delta_time      : self.delta_time,
            size            : Vector2f::new(self.window.size().x as f32, self.window.size().y as f32),
            left_click_state: mouse::Button::Left.is_pressed(),
            mouse_pos       : Vector2f::new(self.window.mouse_position().x as f32, self.window.mouse_position().y as f32),
            size_title_bar  : self.size_title_bar,
        }
    }
    fn draw_title_bar(&mut self)
    {
        //BAR
        let mut my_title_bar : RectangleShape = RectangleShape::new();
        my_title_bar.set_position(Vector2f::new(0.0, 0.0));
        my_title_bar.set_size(Vector2f::new(self.window.size().x as f32, self.size_title_bar.y));
        my_title_bar.set_fill_color(Color::rgb(38,44,50));
        self.window.draw_rectangle_shape(&my_title_bar, &RenderStates::DEFAULT);

        let mut text_title = Text::new(&self.title, &self.font, 12);
        text_title.set_position(self.title_text_position);
        self.window.draw_text(&text_title, &RenderStates::DEFAULT);

        //CLOSE BUTTON
        if self.enabled_close_button
        {
            let mut my_button_close : RectangleShape = RectangleShape::new();
            my_button_close.set_size(Vector2f::new(self.size_title_bar.y, self.size_title_bar.y));
            my_button_close.set_position(Vector2f::new(self.window.size().x as f32 - my_button_close.size().x, 0.0));
            my_button_close.set_fill_color(Color::rgb(255,44,50));
            if self.get_win_info().mouse_pos.x > my_button_close.position().x && self.get_win_info().mouse_pos.x < my_button_close.position().x + my_button_close.size().x
            && self.get_win_info().mouse_pos.y > my_button_close.position().y && self.get_win_info().mouse_pos.y < my_button_close.position().y + my_button_close.size().y
            {
                let nr: f32 = my_button_close.fill_color().r as f32 * 0.5;
                let ng: f32 = my_button_close.fill_color().g as f32 * 0.5;
                let nb: f32 = my_button_close.fill_color().b as f32 * 0.5;
                my_button_close.set_fill_color(Color::rgb(nr as u8, 
                                                        ng as u8, 
                                                        nb as u8));
            }
            self.window.draw_rectangle_shape(&my_button_close, &RenderStates::DEFAULT);
        
            let mut text_close_button = Text::new("X", &self.font, 12);
            text_close_button.set_position(Vector2f::new(my_button_close.position().x + 4.0, 3.0));
            self.window.draw_text(&text_close_button, &RenderStates::DEFAULT);
        }
    }
    pub fn display(&mut self)
    {
        self.draw_title_bar();
        
        self.click_left_up = false;
        if !mouse::Button::Left.is_pressed() && self.click_left_ant
        {
            self.click_left_up = true;
        }
        self.click_left_down = false;
        if mouse::Button::Left.is_pressed() && !self.click_left_ant
        {
            self.click_left_down = true;
        }

        self.click_left_ant = mouse::Button::Left.is_pressed();
        self.window.display();
    }
    fn has_focus(&self) -> bool
    {
        self.window.has_focus()
    }
    fn update_title_bar(&mut self)
    {
        if !self.click_left_ant && mouse::Button::Left.is_pressed()
        &&  (self.window.mouse_position().y as f32) < self.size_title_bar.y
             && self.window.mouse_position().y > 0 
             && self.window.mouse_position().x > 0
            {
                if (self.window.mouse_position().x as f32) > ((self.window.size().x as f32) - self.size_title_bar.y)
                {
                    if self.has_focus()
                    {
                        self.close_button_clicked = true;                    
                    }
                }
                else
                {
                    self.is_dragged = true;
                    self.offset_draw_mouse = mouse::desktop_position() - self.window.position()
                }
            }
        if self.click_left_ant && !mouse::Button::Left.is_pressed()
        && self.is_dragged
        {
            self.is_dragged = false;
        }
        if self.is_dragged
        {
            self.window.set_position(mouse::desktop_position() - self.offset_draw_mouse);
        }
        if self.close_button_clicked && !mouse::Button::Left.is_pressed() && !self.on_closing
        {
            self.close();
            self.close_button_clicked = false;
        }
    }
    pub fn close(&mut self)
    {
        if !self.insta_close
        {
            self.on_closing = true;
        }
        else
        {
            self.window.close();
        }
    }
    pub fn frame(&mut self)
    {
        self.delta_time = self.init_frame_time.elapsed().as_millis() as f32/1000.0;
        self.init_frame_time = Instant::now();
        self.key_up = None;
        self.key_up_char = None;
        self.delta_wheel_mouse = 0;
        // loop 
        // {
        //     let event = self.window.window.poll_event();
        //     if event.is_none()
        //     {
        //         break;
        //     }
        //     if event.unwrap() == sfml::window::Event::Closed // CANCEL CLOSE WITH F4
        //     {
        //         continue;
        //     }
        // }
        while let Some(event) = self.window.poll_event()
        {
            match event
            {
                Event::Closed =>
                {
                    self.close()
                },
                Event::TextEntered { unicode } => 
                {
                    // println!("The key {} has been pressed", unicode);
                    self.key_up_char = Some(unicode);
                },
                Event::KeyPressed { code, .. } =>
                {
                    self.key_up = Some(code);
                },
                Event::MouseWheelScrolled { delta, ..} => 
                {
                    self.delta_wheel_mouse = delta as i8;
                },
                _ => {/*Nothing...*/},
            };

        }
        self.update_title_bar();

        //Drawingw objects...
        self.window.set_active(true);
        self.window.clear(Color::BLACK);
    }
}
impl Window
{
    pub fn get_middle_point_window(&self) -> Vector2f
    {
        Vector2f::new(self.window.size().x as f32/2.0, self.window.size().y as f32/2.0)
    }
}