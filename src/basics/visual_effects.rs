use std::time::Instant;

use sfml::system::Vector2f;

pub struct  VisualEffects;
impl VisualEffects
{
    pub fn calculate_vector_with_time(multiplier: Vector2f, time: Instant) -> Vector2f
    {
        Vector2f::new(time.elapsed().as_secs_f32().cos()*multiplier.x, time.elapsed().as_secs_f32().sin()*multiplier.y)
    }
}