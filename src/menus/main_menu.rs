use std::process::Command;
use std::time::Instant;

use sfml::SfBox;
use sfml::graphics::Color;
use sfml::graphics::RenderTarget;
use sfml::graphics::RectangleShape;
use sfml::graphics::Shape;
use sfml::graphics::RenderStates;
use sfml::graphics::Font;
use sfml::graphics::Text;
use sfml::graphics::Transformable;
use sfml::system::Vector2f;

use crate::basics::level::Level;
use crate::level_loader::level_loader::LevelLoader;
use crate::level_loader::impls::level_loader_sub_window_impl::LevelLoaderSubWindow;

use crate::buttons::arrow_button::ArrowButton;
use crate::buttons::credits_button::CreditsButton;
use crate::buttons::load_level_button::LoadLevelButton;
use crate::buttons::make_level_button::MakeLevelButton;
use crate::buttons::play_button::PlayButton;

use crate::basics::visual_effects::VisualEffects;
use crate::basics::window::Window;

pub struct MainMenu
{
    pub background_color                : Color,
    pub is_paused                       : bool,
    pub title_position_text             : Vector2f,
    pub title_font_size                 : u32,
    pub play_button                     : PlayButton,
    pub credits_button                  : CreditsButton,
    pub make_level_button               : MakeLevelButton,
    pub load_level_button               : LoadLevelButton,
    pub arrow_button_up                 : ArrowButton,
    pub arrow_button_down               : ArrowButton,
    pub font                            : SfBox<Font>,
    pub init_time                       : Instant,
    pub title_movement_multiplier       : Vector2f,
    pub option                          : u8, // 1 -> Game
    pub finished                        : bool,
    pub level_loader_sub_window         : LevelLoaderSubWindow,
    pub is_loading_level                : bool,
    pub levels                          : Vec<Level>,
    pub levels_index                    : usize,
    pub level_path                      : Option<String>
}
impl MainMenu
{
    pub fn new(font: SfBox<Font>, font_for_sub_window: SfBox<Font>) -> MainMenu
    {
        let buttons_movement_multiplier = Vector2f::new(0.0, 0.0);

        MainMenu
        {
            background_color                : Color::rgba(80, 150, 80, 255),
            is_paused                       : false,
            title_position_text             : Vector2f::new(35.0, 180.0),
            title_font_size                 : 41,
            play_button                     : PlayButton::new(Vector2f::new(50.0,300.0), buttons_movement_multiplier),
            make_level_button               : MakeLevelButton::new(Vector2f::new(200.0, 300.0), buttons_movement_multiplier),
            credits_button                  : CreditsButton::new(Vector2f::new(50.0,400.0), buttons_movement_multiplier),
            load_level_button               : LoadLevelButton::new(Vector2f::new(200.0,400.0), buttons_movement_multiplier),
            arrow_button_up                 : ArrowButton::new(Vector2f::new(0.0, Window::get_title_bar_size().y), buttons_movement_multiplier, true),
            arrow_button_down               : ArrowButton::new(Vector2f::new(0.0, Window::get_title_bar_size().y), buttons_movement_multiplier, false), 
            font,          
            init_time                       : Instant::now(),
            title_movement_multiplier       : Vector2f::new(0.0, 5.0),
            option                          : 0,
            finished                        : false,
            level_loader_sub_window         : LevelLoaderSubWindow::new(font_for_sub_window),
            is_loading_level                : false,
            levels                          : Vec::new(),
            levels_index                    : 0,
            level_path                      : None,
        }
    }
    pub fn reset(&mut self)
    {
        self.finished = false;
    }
    pub fn frame(&mut self, win : &Window)
    {
        let mut background = RectangleShape::new();
        background.set_fill_color(self.background_color);
        background.set_size(win.get_win_info().size);
        win.window.draw_rectangle_shape(&background, &RenderStates::DEFAULT);

        let binding = self.font.clone();
        let mut text_title = Text::new("GolfGame", &binding, self.title_font_size);
        text_title.set_position(self.title_position_text + VisualEffects::calculate_vector_with_time(self.title_movement_multiplier, self.init_time));
        text_title.set_fill_color(Color::WHITE);
        text_title.set_outline_thickness(5.0);
        text_title.set_outline_color(Color::BLACK);
        win.window.draw_text(&text_title, &RenderStates::DEFAULT);

        
        let mut copytext = Text::new("Copyright 2023 TEVES", &binding, 10);
        copytext.set_position(Vector2f::new(5.0, win.get_win_info().size.y - 15.0));
        copytext.set_fill_color(Color::WHITE);
        copytext.set_outline_thickness(5.0);
        copytext.set_outline_color(Color::BLACK);
        win.window.draw_text(&copytext, &RenderStates::DEFAULT);
    }
    pub fn buttons_functions(&mut self)
    {
        if !self.is_loading_level
        {

            if self.play_button.button.clickup
            {
                self.finished = true;
                self.option   = 1;
            }
            if self.credits_button.button.clickup
            {
                if cfg!(target_os = "windows")
                {
                    Command::new("cmd")
                            .args(["/C", "start https://www.instagram.com/teves_manu/"])
                            .output()
                            .expect("failed to execute process");
                }
                if cfg!(target_os = "linux")
                {
                    Command::new("bash")
                    .args(["/C", "firefox https://www.instagram.com/teves_manu/"])
                    .output()
                    .expect("failed to execute process");
                }
            }
            if self.make_level_button.button.clickup
            {
                self.finished = true;
                self.option   = 2;
            }
            if self.load_level_button.button.clickup
            {
                self.level_loader_sub_window.reset();
                self.is_loading_level = true;
                self.level_loader_sub_window.window.window.set_visible(true);
                self.levels = LevelLoader::get_all_levels();
            }
        }
    }
    pub fn buttons_update(&mut self, win: &Window)
    {
        self.play_button.button.update(win, self.init_time);
        self.credits_button.button.update(win, self.init_time);
        self.make_level_button.button.update(win, self.init_time);
        self.load_level_button.button.update(win, self.init_time);
    }
    pub fn paused_frame(&mut self, win : &Window)
    {
        let mut paused_effect = RectangleShape::new();
        paused_effect.set_size(win.get_win_info().size);
        paused_effect.set_fill_color(Color::rgba(30,30,30,220));
        win.window.draw_rectangle_shape(&paused_effect, &RenderStates::DEFAULT);
    }
    pub fn update(&mut self, win : &Window)
    {
        self.frame(win);
        
        //UPDATE CLICKs
        if !self.is_loading_level
        {
            self.buttons_functions();
        }
        self.buttons_update(win);

        if self.is_loading_level
        {
            if self.level_loader_sub_window.window.window.is_open()
            {
                self.level_loader_sub_window.frame();
                self.level_loader_sub_window_update();
                self.level_loader_sub_window.display();
            }
            if !self.finished && !self.level_loader_sub_window.window.window.is_open()
            {
                self.is_loading_level = false;
            }
            self.paused_frame(win);
        }
    }
}