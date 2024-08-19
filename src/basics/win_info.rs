use sfml::system::Vector2f;

pub struct WinInfo
{
    pub delta_time      : f32     ,
    pub size            : Vector2f,
    pub left_click_state: bool    ,
    pub mouse_pos       : Vector2f,
    pub size_title_bar  : Vector2f,
}
impl WinInfo
{
    pub fn clone(&self) -> WinInfo
    {
        WinInfo
        {
            delta_time           : self.delta_time,
            size                 : self.size,
            left_click_state     : self.left_click_state,
            mouse_pos            : self.mouse_pos, 
            size_title_bar       : self.size_title_bar, 
        }
    }
}