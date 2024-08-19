use sfml::SfBox;
use sfml::graphics::Color;
use sfml::graphics::IntRect;
use sfml::graphics::RenderTarget;
use sfml::graphics::Font;
use sfml::graphics::Texture;
use sfml::system::Vector2f;
use sfml::system::Vector2i;

use crate::basics::slider_input::Slider;
use crate::game::GameManger;
use crate::game_components::wall::Wall;
use crate::game_components::water::Water;

use crate::buttons::save_button::SaveButton;
use crate::buttons::home_button::HomeButton;

use crate::basics::input_form::InputForm;
use crate::basics::window::Window;
use crate::basics::subwindow::SubWindow;
use crate::basics::popup_message::PopupMessage;

use core::panic;

pub struct LevelMaker
{
    pub left_last_click_state  : bool,
    pub wall                   : Wall,
    pub walls                  : Vec<Wall>,
    pub water                   : Water,
    pub ball_position          : Vector2f,
    pub goal_position          : Vector2f,
    pub posible_position       : Option<Vector2f>,
    pub subwindow              : SubWindow,
    pub subwindow_prop         : SubWindow,
    pub option                 : Option<String>,
    pub popup_message          : PopupMessage,
    pub save_button            : SaveButton,
    pub home_button            : HomeButton,
    pub ball_spawn_texture     : SfBox<Texture>,
    pub finished               : bool,
    pub ball_is_dragged        : bool,
    pub goal_is_dragged        : bool,
    pub offset_on_selection    : Option<Vector2f>,
    pub on_saving              : bool,
    pub font                   : SfBox<Font>,
    pub input_author           : InputForm,
    pub input_project          : InputForm,
    pub size_slider            : Slider,
    pub unabled_dragged_obj    : bool,
}

impl LevelMaker
{
    pub const DRAGGED_COLOR: Color = Color::rgba(255, 255, 255, 128);
    pub const DRAGGED_ERR_COLOR: Color = Color::rgba(255, 0, 0, 128);

    pub fn new(font: SfBox<Font>, win: &mut Window) -> LevelMaker
    {
        //SUBWINDOWS CREATIONS
        let mut subwindow = SubWindow::new(font.clone(), "LEVEL MAKER".to_owned(), Vector2f::new(208.0, 400.0));
        subwindow.set_position(
            Vector2i::new(win.window.position().x + win.get_win_info().size.x as i32,
                          win.window.position().y)
        );
        let mut subwindow_prop = SubWindow::new(font.clone(), "OBJECT PROPERTIES".to_owned(), Vector2f::new(208.0,200.0));
        subwindow_prop.set_position(
            Vector2i::new(win.window.position().x + win.get_win_info().size.x as i32,
                          win.window.position().y + subwindow.window.window.size().y as i32)
        );
        win.window.set_framerate_limit(60);
        let mut ball_spawn_texture: SfBox<Texture> = match Texture::new()
        {
            Some(texture) => texture,
            None => panic!("Err to create textures."),
        };
        if ball_spawn_texture.load_from_file("./assets/Sprites/LVL MAKER/BALL_SPAWN.png", IntRect::new(0, 0, 0, 0)).is_err()
        {
            panic!("Err to load textures");
        }

        LevelMaker
        {
            left_last_click_state  : false,
            wall                   : Wall::new(Vector2f::new(0.0,0.0)),
            walls                  : Vec::new(),
            water                  : Water::new(Vector2f::new(0.0,0.0)),
            ball_position          : win.get_middle_point_window(),
            goal_position          : win.get_middle_point_window(),
            posible_position       : Some(Vector2f::new(0.0, 0.0)),
            subwindow,
            subwindow_prop,
            option                 : None,
            popup_message          : PopupMessage::new(font.clone()),
            save_button            : SaveButton::new(Vector2f::new(0.0,0.0), Vector2f::new(0.0,0.0)),
            home_button            : HomeButton::new(Vector2f::new(0.0,0.0), Vector2f::new(0.0,0.0)),
            ball_spawn_texture,
            finished               : false,
            ball_is_dragged        : false,
            goal_is_dragged        : false,
            offset_on_selection    : None,
            on_saving              : false,
            font,
            input_author           : InputForm::new(),
            input_project          : InputForm::new(),
            size_slider            : Slider::new(),
            unabled_dragged_obj    : false,
        }
    }
    pub fn reset(&mut self, win: &mut Window)
    {
        //GENERAL
        self.finished = false;
        self.walls = Vec::new();
        self.option = None;

        self.on_saving = false;
        self.offset_on_selection = None;

        self.subwindow.reset();
        self.subwindow.window.window.set_visible(true);
        self.subwindow_prop.reset();
        self.subwindow_prop.window.window.set_visible(true);

        //BALL
        self.ball_position = win.get_middle_point_window();
        self.ball_is_dragged = false;

        win.window.request_focus();
    }
    pub fn build(&mut self, win : &mut Window, gm : &mut GameManger)
    {
        //Building
        if self.option.is_some()
        {
            if self.option.clone().unwrap() == *"WALL"
            {
                self.build_wall(win, gm);
            }
            if self.option.clone().unwrap() == *"BALL"
            {
                self.build_ball(win);
            }
        }
        if sfml::window::mouse::Button::Right.is_pressed()
        {
            self.option = None;
        }
    }
    pub fn buttons_update(&mut self, win : &mut Window)
    {
        //Buttons update
        if self.save_button.button.clickup
        {
            // self.save_level();
            self.save_button.button.update(win, std::time::Instant::now());
            self.on_saving = true;
        }
        if self.home_button.button.clickup
        {
            self.subwindow.window.window.close();
            self.subwindow_prop.window.window.close();
            self.finished = true;
        }
    }
    //Esta funcion resetea los valores que se setean durante cada frame y deberian ser nulos por cada frame nuevo
    pub fn reset_update_values(&mut self)
    {
        self.unabled_dragged_obj = false;
        if self.option.is_some()
        {
            for wall in self.walls.iter_mut()
            {
                if wall.selected
                {
                    wall.selected = false;
                }
            }
        }
        if !sfml::window::mouse::Button::Left.is_pressed()
        {
            for wall in self.walls.iter_mut()
            {
                if wall.dragged
                {
                    wall.dragged = false;
                }
            }
        }
    }
    pub fn update(&mut self, win: &mut Window, gm: &mut GameManger)
    {
        //UPDATES
        self.reset_update_values();
        if !self.on_saving && self.option.is_none()
        {
            self.update_all_drag_system(win, gm);
        }

        self.build(win, gm);

        self.buttons_update(win);

        for wall in self.walls.iter_mut()
        {
            if wall.selected
            {
                let mut color_ = LevelMaker::DRAGGED_COLOR;
                if self.unabled_dragged_obj
                {
                    color_ = LevelMaker::DRAGGED_ERR_COLOR;
                }
                wall.color = Some(color_);
            }
            else
            {
                wall.color = None;
            }
            wall.draw(win);
        }
        gm.ball.draw(win);
        self.water.position = win.get_win_info().mouse_pos;
        // self.water.draw(win);
        self.water.radius += win.delta_wheel_mouse as f32 * win.delta_time * 100.0;
        self.popup_message.update(win);
        self.left_last_click_state = sfml::window::mouse::Button::Left.is_pressed();
        self.goal_position = gm.goal.position;
        self.subwindow_update();
        self.subwindow_save_update(win);
        self.subwindow_properties_update();
    }
}