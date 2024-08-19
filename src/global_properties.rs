use sfml::graphics::Color;

pub struct GlobalProperties{}
impl GlobalProperties
{
    pub const BACKGROUND_SEMITRANSPARENT : Color = Color::rgba(30, 30, 30, 200);
    pub const BACKGROUND_COLOR_SECUNDARY_WINDOW : Color = Color::rgb(0, 74, 135);
    pub const MAX_LENGHT_NAME : usize = 12;
    pub const MIN_WALL_SCALE  : f32   = 0.5;
    pub const MAX_WALL_SCALE  : f32   = 2.0;
}