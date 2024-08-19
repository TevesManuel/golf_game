use std::fs;

use sfml::system::Vector2f;
use sfml::system::Vector3i; 
use sfml::graphics::Transformable;

use rand::Rng;

use crate::basics::obstacles::Obstacles;
use crate::basics::level::Level;
use crate::game::GameManger;

pub struct LevelLoader
{
    pub obstacles       : Obstacles,
    pub ball_position   : Vector2f,
    pub goal_position   : Vector2f,
    pub position        : Vector2f,
    pub scale           : Vector2f,
    pub color           : Vector3i,
    pub getting_number  : bool,
    pub number          : String,
}
impl LevelLoader
{
    pub fn new() -> LevelLoader
    {
        LevelLoader
        {
            obstacles      : Obstacles::new(),
            ball_position  : Vector2f::new(0.0,0.0),
            goal_position  : Vector2f::new(0.0,0.0),
            position       : Vector2f::new(-1.0, -1.0),
            scale          : Vector2f::new(-1.0, -1.0),
            color          : Vector3i::new(-1, -1, -1),
            getting_number : false,
            number         : "".to_owned(),
        }
    }
    pub fn intern_reset(&mut self)
    {
        self.position       = Vector2f::new(-1.0, -1.0);
        self.scale          = Vector2f::new(-1.0, -1.0);
        self.color          = Vector3i::new(-1, -1, -1);
        self.getting_number = false;
        self.number         = "".to_owned();
    }
    //LEVELS
    fn get_all_levels_paths() -> Vec<String>
    {
        let mut files = Vec::new();
        for file in fs::read_dir("./Data").unwrap()
        {
            files.push(file.unwrap().path().as_os_str().to_os_string().into_string().unwrap());
        }

        files
    }
    //RESETS
    pub fn reset(&mut self)
    {
        self.obstacles.reset();
    }
    //LEVELS
    pub fn get_all_levels() -> Vec<Level>
    {
        let mut levels = Vec::new();
        let files = LevelLoader::get_all_levels_paths();
        for file in files
        {
            let mut level = Level::new();
            level.path = file.clone();
            match fs::read_to_string(file)
            {
                Ok(content) =>
                {
                    for line in content.split('\n')
                    {
                        if line.contains("PROJECT_NAME")
                        {
                            let mut oncopy = false;
                            for ch in line.chars()
                            {
                                if ch == '\"'
                                {
                                    oncopy = true;
                                }
                                else if oncopy
                                {
                                    level.project_name.push(ch);
                                }
                            }
                        }
                        else if line.contains("AUTHOR_NAME")
                        {
                            let mut oncopy = false;
                            for ch in line.chars()
                            {
                                if ch == '\"'
                                {
                                    oncopy = true;
                                }
                                else if oncopy
                                {
                                    level.author_name.push(ch);
                                }
                            }
                        }
                    }
                },
                Err(_) => panic!("Err to open level file."),
            }
            levels.push(level);
        }
        
        levels
    }
    fn internal_load_level(&mut self, path : String, gm : &mut GameManger)
    {
        gm.path_of_level = path.clone();
        let contents = match fs::read_to_string(path)
        {
            Ok(content) => content,
            Err(_) => panic!("Error loading levels"),
        };
        for line in contents.split('\n')
        {
            self.decode_line(line.to_lowercase());
        }
        gm.ball.posible_new_shape.set_position(self.ball_position);
        gm.ball.application_of_position();
        gm.goal.position = self.goal_position;
    }
    pub fn load_random_level(&mut self, gm: &mut GameManger)
    {
        let ca = fs::read_dir("./Data").unwrap().count();
        let index = rand::thread_rng().gen_range(0..=(ca-1));
        self.internal_load_level(LevelLoader::get_all_levels_paths().get(index).unwrap().to_string(), gm);
    }
    pub fn reload_level(&mut self, gm : &mut GameManger)
    {
        self.internal_load_level(gm.path_of_level.clone(), gm);
    }
    pub fn load_level(&mut self, gm: &mut GameManger, path: String)
    {
        self.internal_load_level(path, gm);
    }
}