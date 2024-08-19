use app::App;
use level_maker::level_maker::LevelMaker;
use sfml::audio::Sound; 
use sfml::audio::SoundBuffer;
use sfml::system::Vector2f;
use sfml::system::Vector2i; 
use sfml::graphics::Font;

mod basics
{
    pub mod level;
    pub mod physics;
    pub mod win_info;
    pub mod button;
    pub mod subwindow;
    pub mod window;
    pub mod popup_message;
    pub mod charge_animation;
    pub mod visual_effects;
    pub mod input_form;
    pub mod slider_input;
    pub mod obstacles;
}

use crate::basics::window::Window;
use basics::charge_animation::ChargeAnimation;

mod game_components
{
    pub mod ball;
    pub mod background;
    pub mod pointing;
    pub mod goal;
    pub mod trajectory_viewer;
    pub mod wall;
    pub mod water;
}

use crate::game_components::background::Background;
use crate::game_components::pointing::Pointing;
use crate::game_components::goal::Goal;

mod game;
use game::GameManger;
mod global_properties;

mod menus
{
    pub mod main_menu_subwindow_load_impl;
    pub mod main_menu;
    pub mod pause_menu;
    pub mod win_menu;
}

mod buttons
{
    pub mod credits_button;
    pub mod arrow_button;
    pub mod home_button;
    pub mod load_level_button;
    pub mod make_level_button;
    pub mod play_button;
    pub mod save_button;
    pub mod replay_button;
}

mod level_loader
{
    pub mod level_loader;
    pub mod impls
    {
        pub mod level_loader_main_decoder_impl;
        pub mod level_loader_wall_decoder;
        pub mod level_loader_sub_window_impl;
    }
}
use crate::level_loader::level_loader::LevelLoader;

mod level_maker
{
    pub mod level_maker;
    mod impls
    {
        pub mod level_maker_sub_window_impl;
        pub mod level_maker_sub_window_save_impl;
        pub mod level_maker_drag_impl;
        pub mod level_maker_save_impl;
        pub mod level_maker_builders_impl;
        pub mod level_maker_properties_subwin_impl;
        pub mod level_maker_basics_impl;
    }
}

use crate::menus::main_menu::MainMenu;
use crate::menus::pause_menu::PauseMenu;

pub mod app;

fn main()
{
    //Window configs
    let width                  : f32      = 400.0;
    let height                 : f32      = 600.0;
    let title                  : &str     = "TEVES | GolfGame";

    let background_offset      : Vector2i = Vector2i::new(0,-Window::get_title_bar_size().y as i32);
    
    let force_multiplier       : f32      = 1.5;
    //Sounds parameters
    let swing_sound_path       : &str     = "./assets/Audio/Swing.ogg";

    let win_sound_path       : &str     = "./assets/Audio/win.ogg";
    //Animation parameters
    let vel_charge_animation   : f32      = 600.0;

    let sound_buffer_swing = match SoundBuffer::from_file(swing_sound_path)
    {
        Ok(sb) => sb,
        Err(_) => panic!("Error on load sound files."),
    };
    let mut golf_swing = Sound::new();
    golf_swing.set_buffer(&sound_buffer_swing);
    
    let sound_buffer_win = match SoundBuffer::from_file(win_sound_path)
    {
        Ok(sb) => sb,
        Err(_) => panic!("Error on load sound files."),
    };
    let mut win_sound = Sound::new();
    win_sound.set_buffer(&sound_buffer_win);

    let font = match Font::from_file("./assets/Fonts/m38.TTF")
    {
        Some (x) => x,
        None     => panic!("Error to load font.")
    };

    let font_game = match Font::from_file("./assets/Fonts/font.ttf")
    {
        Some (x) => x,
        None     => panic!("Error to load font.")
    };

    let mut win                      = Window::new(String::from(title), Vector2f::new(width, height), font.clone());
    
    //Init GameObject
    let background               = Background::new(background_offset);
    let pointing                 = Pointing::new(force_multiplier, golf_swing);
    let goal                     = Goal::new(win_sound);
    
    let game = GameManger::new(&win, goal, pointing, background, font_game.clone());
    
    let mut level_maker = LevelMaker::new(font.clone(), &mut win);

    let mut charge_animation = ChargeAnimation::new(vel_charge_animation);
    let mut pause_menu       = PauseMenu::new(font_game.clone());

    let mut level_loader     = LevelLoader::new();

    let mut main_menu        = MainMenu::new(font_game, font);

    let stage   : String   = "Menu".to_string();//Menu; Game; LVL MAKER; LVL LOADER; COMMUNITY

    let swiching: bool   = false;
    
    let mut app = App
    {
        win              : &mut win,
        gm               : game,
        lvl_maker        : &mut level_maker,
        lvl_loader       : &mut level_loader,
        main_menu        : &mut main_menu,
        pause_menu       : &mut pause_menu,
        charge_animation : &mut charge_animation,
        died: false,
        stage,
        to: "".to_string(),
        swiching,
    };
    app.run();
}
