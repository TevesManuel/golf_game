use sfml::window::mouse;
use sfml::graphics::Shape;
use sfml::graphics::Transformable;
use crate::basics::window::Window;
use crate::game_components::goal::Goal;
use crate::game::GameManger;
use crate::level_maker::level_maker::LevelMaker;
use crate::game_components::ball::Ball;

impl LevelMaker
{
    pub fn update_all_drag_system(&mut self, win: &mut Window, gm : &mut GameManger)
    {
        self.wall_drag_update(win, gm);
        self.ball_drag_update(win, &mut gm.ball);
        self.goal_drag_update(win, &mut gm.goal);
    }
    pub fn is_selected_any_wall(&mut self) -> bool
    {
        for wall in &self.walls
        {
            if wall.selected
            {
                return true;
            }
        }
        false
    }
    pub fn wall_drag_update(&mut self, win: &mut Window, gm : &mut GameManger)
    {
        //<-DRAG SYSTEM
        //Listen to remove selection
        if win.click_left_down && win.window.has_focus()
        {
            for wall_index in 0..self.walls.len()
            {
                if self.walls[wall_index].selected
                && win.get_win_info().mouse_pos - self.offset_on_selection.unwrap() != self.walls[wall_index].pos
                {
                    if self.wall_check_coll_with_any(&self.walls[wall_index], gm)
                    {
                        self.walls[wall_index].died = true;
                    }
                    self.walls[wall_index].selected = false;
                }
            }
        }
        //LISTEN TO SELECTION
        if win.click_left_down
        {
            for wall in self.walls.iter_mut()
            {
                if wall.is_over(win.get_win_info().mouse_pos)
                {
                    wall.selected = true;
                    wall.dragged = true;
                    self.offset_on_selection = Some(win.get_win_info().mouse_pos - wall.pos);
                }
            }
        }
        //MOVEMENT
        for wall_index in 0..self.walls.len()
        {
            if self.walls[wall_index].selected
            && self.wall_check_coll_with_any(&self.walls[wall_index], gm)
            {
                self.unabled_dragged_obj = true;
            }
        }
        for wall in self.walls.iter_mut()
        {
            if wall.dragged
            {
                if self.offset_on_selection.is_some()
                {
                    wall.pos = win.get_win_info().mouse_pos - self.offset_on_selection.unwrap();
                }
                else
                {
                    panic!("Err to select an object");
                }
            }
        }
        /*
        --------------------------
             Funcion Retain
        --------------------------
        Recorre todos los elementos y brinda una referencia inmutable, si devolvemos true la
        referencia se mantiene el vector, si se devuelve false se elimina dicho elemento de la estructura
        */
        self.walls.retain(|wall| {
            if (wall.dragged && mouse::Button::Right.is_pressed()) || wall.died
            {
                return false;
            }
            true
        });
    }
    pub fn ball_drag_update(&mut self, win: &mut Window, ball : &mut Ball)
    {  
        //Off drag system
        if win.click_left_up
        {
            self.ball_is_dragged = false;
        }

        if self.ball_check_coll_with_any(ball)
        {
            self.unabled_dragged_obj = true;
            // println!("asd");
        }
        if self.ball_is_dragged
        {
            self.ball_position = win.get_win_info().mouse_pos;
        }
        if mouse::Button::Left.is_pressed()
        && !self.left_last_click_state
        && !self.ball_is_dragged
        && ball.is_over(win.get_win_info().mouse_pos)
        {
            self.ball_is_dragged = true;
        }   
        
        //Color
        {
            if self.ball_is_dragged
            {
                let mut color = ball.from_color;
                color.a = 128;
                ball.color = Some(color);
                let mut color = ball.shape.outline_color();
                color.a = 128;
                ball.color_thickness = Some(color);
            }
            else
            {
                let mut color = ball.from_color;
                color.a = 255;
                ball.color = Some(color);
                let mut color = ball.shape.outline_color();
                color.a = 255;
                ball.color_thickness = Some(color);    
            }
            if self.unabled_dragged_obj
            {
                ball.color = Some(LevelMaker::DRAGGED_ERR_COLOR);
                let mut color = ball.shape.outline_color();
                color.a = 128;
                ball.color_thickness = Some(color);
            }
        }

        ball.posible_new_shape.set_position(self.ball_position);
        ball.shape.set_position(self.ball_position);
    }
    pub fn goal_drag_update(&mut self, win: &mut Window, goal: &mut Goal)
    {
        if win.get_win_info().mouse_pos.x > goal.get_shape().global_bounds().left
        && win.get_win_info().mouse_pos.x < goal.get_shape().global_bounds().left + goal.get_shape().global_bounds().width
        && win.get_win_info().mouse_pos.y > goal.get_shape().global_bounds().top
        && win.get_win_info().mouse_pos.y < goal.get_shape().global_bounds().top + goal.get_shape().global_bounds().height
        && sfml::window::mouse::Button::Left.is_pressed()
        && !self.left_last_click_state
        {
            self.goal_is_dragged = true;
        }
        if !sfml::window::mouse::Button::Left.is_pressed()
        {
            self.goal_is_dragged = false;
        }
        if self.goal_is_dragged
        {
            goal.position = win.get_win_info().mouse_pos;
        }
    }
}