use sfml::SfBox;
use sfml::graphics::{Font, Transformable};

use crate::basics::obstacles::Obstacles;
use crate::basics::window::Window;

use crate::game_components::ball::Ball;
use crate::game_components::background::Background;
use crate::game_components::pointing::Pointing;
use crate::game_components::wall::Wall;
use crate::game_components::goal::Goal;
use crate::game_components::trajectory_viewer::TrajectoryViewer;
use crate::menus;
use crate::menus::win_menu::WinMenu;

pub struct GameManger<'s>
{
    pub background       : Background,
    pub ball             : Ball,
    pub trajectory_viewer: TrajectoryViewer, 
    pub pointing         : Pointing<'s>,
    pub goal             : Goal<'s>,
    pub win_menu         : WinMenu,
    pub unable_pause     : bool,
    pub state            : u8,
    pub path_of_level    : String,
}
impl GameManger<'_>
{
    pub fn new<'a>(win: &Window, goal: Goal<'a>, pointing: Pointing<'a>, background: Background, font : SfBox<Font>) -> GameManger<'a>
    {
        let ball                     = Ball::new(win.get_middle_point_window(), sfml::graphics::Color::rgb(255, 255, 255));
        let trajectory_viewer        = TrajectoryViewer::new();
        
        GameManger
        {
            background,
            ball,
            trajectory_viewer,
            pointing,
            goal,
            win_menu         : menus::win_menu::WinMenu::new(font, win),
            unable_pause     : false,
            state            : 0,
            path_of_level    : String::new(),
        }
    }
    pub fn update_ball_position(&mut self, win: &Window, walls: &[Wall])
    {   
        for wall in walls.iter()
        {
            match wall.detect_colision(self.ball.posible_new_shape.clone(), self.ball.shape.clone())
            {
                1 =>
                {
                    self.ball.physics.force.x *= -1.0;
                    self.ball.update(win.get_win_info());
                },
                2 =>
                {
                    self.ball.physics.force.y *= -1.0;
                    self.ball.update(win.get_win_info());
                },
                _ => {},
            };
        }
        self.ball.application_of_position();
    }
    pub fn reset(&mut self)
    {
        self.ball.reset();
        self.goal.reset();
        self.pointing.reset();
        self.win_menu.reset();
        self.unable_pause = false;
        self.state = 0;
    }
    pub fn update(&mut self, win: &Window, obstacles: &Obstacles, input_enabled: bool)
    {
        if input_enabled && self.ball.on_goal.is_none()
        {
            //Bola golpeada?
            if self.pointing.update(win)
            {
                self.ball.last_position = self.ball.shape.position();
            }
            self.trajectory_viewer.update(self.pointing.position_from, win.get_win_info().mouse_pos);
        }
        self.ball.physics.force += self.pointing.force_out;
        self.ball.update(win.get_win_info());
        self.update_ball_position(win, &obstacles.walls);
        self.background.draw(win);
        self.goal.draw(win);
        self.ball.draw(win);

        if input_enabled
        && self.goal.check(self.ball.shape.clone(), self.ball.physics.get_average_force()) && self.ball.on_goal.is_none()
        {
            // println!("\n\nEntro\n\n");
            self.ball.on_goal = Some(std::time::Instant::now());
            self.goal.sound_ptr.play();
        }

        if self.pointing.pointing
        {
            self.trajectory_viewer.draw(win);
        }

        //Colisionables
        for wall in obstacles.walls.iter()
        {
            wall.draw(win);
        }
        for water in obstacles.waters.iter()
        {
            water.draw(win);
            if water.is_col(self.ball.shape.clone())
            {
                self.ball.posible_new_shape.set_position(self.ball.last_position);
                self.ball.physics.force = sfml::system::Vector2f::new(0.0, 0.0);
            }
        }

        self.unable_pause = self.ball.on_goal.is_some();

        if self.ball.finished_goal_anim
        {
            match self.win_menu.update(win, self.pointing.hits)
            {
                Some(out) =>
                {
                    self.state = out;
                    self.goal.sound_ptr.stop();
                },
                None => self.state = 0,
            };
        }
    }
}