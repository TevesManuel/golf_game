use crate::basics::charge_animation::ChargeAnimation;
use crate::basics::window::Window;
use crate::game::GameManger;
use crate::level_loader::level_loader::LevelLoader;
use crate::level_maker::level_maker::LevelMaker;
use crate::menus::main_menu::MainMenu;
use crate::menus::pause_menu::PauseMenu;

pub struct App<'a>
{
    pub win              : &'a mut Window,
    pub gm               : GameManger<'a>,
    pub lvl_maker        : &'a mut LevelMaker,
    pub lvl_loader       : &'a mut LevelLoader,
    pub main_menu        : &'a mut MainMenu,
    pub pause_menu       : &'a mut PauseMenu,
    pub charge_animation : &'a mut ChargeAnimation,
    pub died             : bool,
    pub stage            : String,
    pub to               : String,
    pub swiching         : bool,
}
impl App<'_>
{
    pub fn main_window_system_update(&mut self)
    {
        //Main Update
        if self.win.on_closing && !self.died
        {
            self.charge_animation.init(true, self.win);
            self.died = true;
        }
        if self.win.on_closing && self.charge_animation.finished
        {
            self.win.window.close();
        }
        
        self.win.frame();
    }
    pub fn in_game_update(&mut self)
    {
        //Auto pause when user swich window
        if !self.win.window.has_focus()
        {
            self.pause_menu.set_pause(true);
        }

        //Update for ,gamemanager
        self.gm.update(self.win, &self.lvl_loader.obstacles, !self.pause_menu.is_paused 
            && !self.win.on_closing
            && (self.stage != "LVL MAKER"));

        if self.stage == "Game"
        {
            if !self.gm.unable_pause
            {
                self.pause_menu.update(self.win);
            }
            if self.gm.state == 2 && !self.swiching // Replay
            {
                self.charge_animation.init(true, self.win);
                self.swiching = true;
                self.to = "Replay".to_owned();
            }
            if (self.pause_menu.home_button.button.clickup || self.gm.state == 1) && !self.swiching
            {
                self.charge_animation.init(true, self.win);
                self.swiching = true;
                self.to = "Menu".to_string();
            }
            if self.swiching && self.charge_animation.finished
            {
                if self.to == "Menu"
                {
                    self.swiching = false;
                    self.stage = "Menu".to_string();
                    self.main_menu.reset();    
                    self.charge_animation.init(false, self.win);
                    self.to = "".to_string();
                }
                if self.to == "Replay"
                {
                    self.swiching = false;
                    self.gm.reset();
                    self.lvl_loader.reload_level(&mut self.gm);
                    self.charge_animation.init(false, self.win);
                    self.to = "".to_string();
                }
            }
        }
    }
    pub fn in_lvl_mk_update(&mut self)
    {
        self.lvl_maker.update(self.win, &mut self.gm);
    
        if self.lvl_maker.finished && !self.swiching
        {
            self.charge_animation.init(true, self.win);
            self.swiching = true;
        }
        if self.swiching && self.charge_animation.finished
        {
            self.swiching = false;
            self.stage = "Menu".to_string();
            self.charge_animation.init(false, self.win);
            self.main_menu.reset();
        }
    }
    pub fn in_main_menu_update(&mut self)
    {
        self.main_menu.update(self.win);
        if self.main_menu.finished && !self.swiching
        {
            self.swiching = true;
            self.charge_animation.init(true, self.win);
        }
        if self.charge_animation.finished && self.swiching
        {
            self.lvl_loader.reset();
            match self.main_menu.option
            {
                1 => 
                {
                    self.gm.reset();
                    if self.main_menu.level_path.is_some()
                    {
                        self.lvl_loader.load_level(&mut self.gm, self.main_menu.level_path.clone().unwrap());
                    }
                    else
                    {
                        self.lvl_loader.load_random_level(&mut self.gm);
                    }
                    self.stage = "Game".to_string();
                    self.pause_menu.is_paused = false;
                },
                2 => 
                {
                    self.lvl_maker.reset(self.win);
                    self.gm.reset();

                    self.stage = "LVL MAKER".to_string();
                }
                _ => {panic!("No planified instance.");}
            }
            self.charge_animation.init(false, self.win);
            self.swiching = false;
        }
    }
    pub fn run(&mut self)
    {
        self.charge_animation.init(false, self.win);

        //Main loop
        while self.win.window.is_open()
        {
            self.main_window_system_update();

            if self.stage == "Game" || self.stage == "LVL MAKER"
            {
                self.in_game_update();
                if self.stage == "LVL MAKER"
                {
                    self.in_lvl_mk_update();
                }
            }
            else if self.stage == "Menu"
            {
                self.in_main_menu_update();
            }
            self.charge_animation.update(self.win);
            self.win.display();
        }
    }
}