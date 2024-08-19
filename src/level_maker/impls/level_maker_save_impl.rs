use std::fs::File;
use std::io::Write;

use sfml::system::Vector2f;

use crate::game_components::wall::Wall;
use crate::level_maker::level_maker::LevelMaker;

impl LevelMaker
{
    fn encode_line_pos(&self, name: &str, position : Vector2f) -> String
    {
        let mut out : String;
        out  = String::from(name);
        out += &String::from("[");
        out += &position.x.to_string();
        out += &String::from(", ");
        out += &position.y.to_string();
        out += &String::from("]\n");
        out
    }
    fn encode_line_wall(&self, name: &str, wall : &Wall) -> String
    {
        let mut out : String;
        out  = String::from(name);
        out += &String::from("[");
        out += &wall.pos.x.to_string();
        out += &String::from(", ");
        out += &wall.pos.y.to_string();
        if wall.scale.is_some()
        {
            out += ", ";
            out += &wall.scale.unwrap().x.to_string();
            out += ", ";
            out += &wall.scale.unwrap().y.to_string();
            if wall.color.is_some()
            {
                out += ", ";
                out += &wall.color.unwrap().r.to_string();
                out += ", ";
                out += &wall.color.unwrap().g.to_string();
                out += ", ";
                out += &wall.color.unwrap().b.to_string();
                out += ", ";
            }
        }
        out += &String::from("]\n");
        out
    }

    pub fn save_level(&mut self)
    {
        let mut out : String = String::from("PROJECT_NAME \"");
        out += &self.input_author.text;
        out += "\"\nAUTHOR_NAME \"";
        out += &self.input_project.text;
        out += "\"\n";
        let mut filename : String = self.input_project.text.clone();
        filename += "_";
        filename += &self.input_author.text;
        let mut file_path: String = String::from("./Data/");
        file_path += &filename; 
        file_path += ".TEVES";
        for wall in &self.walls
        {
            out += &self.encode_line_wall("Wall", wall);
        }
        
        out += &self.encode_line_pos("Ball", self.ball_position);
        out += &self.encode_line_pos("Goal", self.goal_position);

        let mut file = match File::create(file_path.as_str())
        {
            Ok(file) => file,
            Err(_) => panic!("Err to open dest file"),
        };

        if file.write_all(out.as_bytes()).is_err()
        {
            panic!("Err to save level.");
        }

        self.on_saving = false;
        self.popup_message.init("saved".to_owned());
    }
}
