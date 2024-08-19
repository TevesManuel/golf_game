pub struct Level
{
    pub project_name : String,
    pub author_name  : String,
    pub path         : String,
}
impl Level
{
    pub fn new() -> Level
    {
        Level
        {
            project_name : String::from(""),
            author_name  : String::from(""),
            path         : String::from(""),
        }
    }
}