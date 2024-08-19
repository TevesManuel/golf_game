use sfml::graphics::Color;

use crate::game_components::water::Water;
use crate::level_loader::level_loader::LevelLoader;
use crate::game_components::wall::Wall;

impl LevelLoader
{
    pub fn decode_line(&mut self, line: String)
    {
        self.intern_reset();
        if line.contains("ball") || line.contains("goal")
        {
            for x in line.chars()
            {
                if x == '['
                {
                    self.getting_number = true;
                }
                else if self.getting_number && x == ','
                {
                    self.position.x = self.number.parse::<f32>().unwrap();
                    self.number = String::from("");
                }
                else if self.getting_number && x == ']'
                {
                    self.position.y = self.number.parse::<f32>().unwrap();
                    break
                }
                else if self.getting_number && x != ' '
                {
                    self.number += String::from(x).as_str();    
                }
            }
            if line.contains("ball")
            {
                self.ball_position = self.position;
            }
            else
            {
                self.goal_position = self.position;    
            }
        }
        if line.contains("wall")
        {
            for x in line.chars()
            {
                if x == '['
                {
                    self.getting_number = true;
                }
                else if self.getting_number && ( x == ',' || x == ']' )
                {
                    self.wall_decoder();
                }
                else if self.getting_number && x != ' '
                {
                    self.number += String::from(x).as_str();    
                }
            }
            let mut wall = Wall::new(self.position);
            if self.scale.x != -1.0 && self.scale.y != -1.0
            {
                wall.scale = Some(self.scale);
            }
            if self.color.x != -1 && self.color.y != -1 && self.color.z != -1
            {
                wall.color = Some(Color::rgb(self.color.x as u8, self.color.y as u8, self.color.z as u8));
            }
            self.obstacles.walls.push(wall);
        }
        if line.contains("water")
        {
            for x in line.chars()
            {
                if x == '['
                {
                    self.getting_number = true;
                }
                else if self.getting_number && ( x == ',' || x == ']' )
                {
                    if self.position.x == -1.0
                    {
                        self.position.x = self.number.parse::<f32>().unwrap();
                    }
                    else if self.position.y == -1.0
                    {
                        self.position.y = self.number.parse::<f32>().unwrap();
                    }
                    else if self.scale.x == -1.0
                    {
                        self.scale.x = self.number.parse::<f32>().unwrap();
                    }
                    self.number = String::from("");
                }
                else if self.getting_number && x != ' '
                {
                    self.number += String::from(x).as_str();    
                }
            }
            let mut water = Water::new(self.position);
            if self.scale.x != -1.0
            {
                water.radius = self.scale.x;
            }
            self.obstacles.waters.push(water);
        }

    }
}