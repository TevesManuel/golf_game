use crate::game::GameManger;
use crate::game_components::ball::Ball;
use crate::level_maker::level_maker::LevelMaker;
use crate::game_components::wall::Wall;

impl LevelMaker
{
    pub fn wall_check_coll_with_any(&self, wall: &Wall, gm: &GameManger) -> bool
    {
        for wall_index in 0..self.walls.len()
        {
            if wall.is_coll_wall(&self.walls[wall_index])
            && !std::ptr::eq(wall, &self.walls[wall_index])
            {
                return true;
            }
        }
        return wall.is_coll(gm.ball.shape.clone()) || wall.is_coll(gm.goal.get_shape());
    }
    pub fn ball_check_coll_with_any(&self, ball: &Ball) -> bool
    {
        for wall in self.walls.iter()
        {
            if wall.is_coll(ball.shape.clone())
            {
                return true;
            }
        }
        false
    }
}