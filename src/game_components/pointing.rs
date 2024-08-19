use sfml::system::Vector2f;
use sfml::audio::Sound;
use crate::basics::window::Window;

pub struct Pointing<'s>
{
    pub position_from       : Vector2f,
    pub pointing            : bool,
    pub force_out           : Vector2f,
    pub force_multiplier    : f32,
    pub sound_ptr           : Sound<'s>,
    pub hits                : u8,
}
impl Pointing<'_>
{
    pub fn new(force_multiplier : f32, sound_ptr : Sound<'_>) -> Pointing
    {
        Pointing 
        { 
            position_from        : Vector2f::new(0.0,0.0),
            pointing             : false,
            force_out            : Vector2f::new(0.0,0.0),
            force_multiplier,
            sound_ptr,
            hits                 : 0,
        }
    }
    pub fn reset(&mut self)
    {
        self.hits = 0;
    }
    pub fn update(&mut self, win: &Window) -> bool
    {
        self.force_out = Vector2f::new(0.0,0.0);
        if win.click_left_down
        {
            self.position_from = win.get_win_info().mouse_pos;
            self.pointing = true;
        }
        if win.click_left_up
        {
            self.pointing = false;
            self.force_out += Vector2f::new(self.force_multiplier*(self.position_from.x - win.get_win_info().mouse_pos.x),
                                                                   self.force_multiplier*(self.position_from.y - win.get_win_info().mouse_pos.y));
            if self.force_out.x.abs() > 1.0 || self.force_out.y.abs() > 1.0
            {
                self.sound_ptr.play();
                self.hits += 1;
            }
            return true;
        }
        false
    }
}