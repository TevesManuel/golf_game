use crate::level_loader::level_loader::LevelLoader;

impl LevelLoader
{
    pub fn wall_decoder(&mut self)
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
        else if self.scale.y == -1.0
        {
            self.scale.y = self.number.parse::<f32>().unwrap();
        }
        else if self.color.x == -1
        {
            self.color.x = self.number.parse::<f32>().unwrap() as i32;
        }
        else if self.color.y == -1
        {
            self.color.y = self.number.parse::<f32>().unwrap() as i32;
        }
        else if self.color.z == -1
        {
            self.color.z = self.number.parse::<f32>().unwrap() as i32;
        }
        self.number = String::from("");
    }
}