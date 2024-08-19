use sfml::window::mouse;

use crate::game::GameManger;
use crate::game_components::wall::Wall;
use crate::level_maker::level_maker::LevelMaker;
use crate::basics::window::Window;

impl LevelMaker
{
    pub fn build_wall(&mut self, win: &Window, gm : &GameManger)
    {
        self.wall.pos   = win.get_win_info().mouse_pos;
        
        if self.wall_check_coll_with_any(&self.wall, gm)
        {
            self.unabled_dragged_obj = true;
        }
        if !self.unabled_dragged_obj
        {
            for wall in self.walls.iter()
            {
                if self.wall.is_coll_wall(wall)
                {
                    self.unabled_dragged_obj = true;
                }
            }
        }
        
        self.posible_position = Some(self.wall.pos);

        if self.posible_position.is_some()
        && !mouse::Button::Left.is_pressed()
        && self.left_last_click_state
        && win.window.has_focus()
        && !self.unabled_dragged_obj
        {
            self.walls.push(Wall::new(self.posible_position.unwrap()));
            self.posible_position = None;
        }
        
        if self.unabled_dragged_obj
        {
            self.wall.color = Some(LevelMaker::DRAGGED_ERR_COLOR);
        }
        else
        {
            self.wall.color = Some(LevelMaker::DRAGGED_COLOR);
        }
        self.wall.draw(win);
    }
    pub fn build_ball(&mut self, win: &Window)
    {
        let mut not_over = true;
        if mouse::Button::Left.is_pressed()
        {
            for wall in &self.walls
            {
                if win.get_win_info().mouse_pos.x > wall.pos.x && win.get_win_info().mouse_pos.x < wall.pos.x + wall.texture.size().x as f32
                && win.get_win_info().mouse_pos.y > wall.pos.y && self.wall.pos.y < win.get_win_info().mouse_pos.y + wall.texture.size().y as f32
                {
                    not_over = false;
                }
            }
            if not_over
            {
                self.ball_position = win.get_win_info().mouse_pos;
            }
        }
        self.option = None;
    }
}