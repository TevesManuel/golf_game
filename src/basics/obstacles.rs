use crate::game_components::wall::Wall;
use crate::game_components::water::Water;

pub struct Obstacles
{
    pub walls: Vec<Wall>,
    pub waters: Vec<Water>,
}
impl Obstacles
{
    pub fn new() -> Obstacles
    {
        Obstacles
        {
            walls: Vec::new(),
            waters: Vec::new(),
        }
    }
    pub fn reset(&mut self)
    {
        self.walls  = Vec::new();
        self.waters = Vec::new();
    }
}