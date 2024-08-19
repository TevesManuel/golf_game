use sfml::system::Vector2f;

pub struct Physics
{
    pub force: Vector2f,
    pub mass: f32,
    pub friction: f32,
}

impl Physics
{
    pub fn new() -> Physics
    {
        Physics
        {
            force    : Vector2f::new(0.0, 0.0),
            mass     : 1.0,
            friction : 0.8,
        }
    }
    pub fn get_average_force(&self) -> f32
    {
        self.force.x * self.force.y
    }
}