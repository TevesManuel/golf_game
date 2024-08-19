use sfml::graphics::CircleShape;
use sfml::graphics::Transformable;
use sfml::graphics::Sprite;
use sfml::graphics::Texture;
use sfml::graphics::IntRect;
use sfml::graphics::RenderStates;
use sfml::graphics::RenderTarget;
use sfml::graphics::Color;
use sfml::graphics::Shape;
use sfml::system::Vector2f;
use sfml::SfBox;
use crate::basics::window::Window;

pub struct Wall
{
    pub texture      : SfBox<Texture>,
    pub pos          : Vector2f,
    pub dragged      : bool,
    pub selected     : bool,
    pub died         : bool,
    pub color        : Option<Color>,
    pub scale        : Option<Vector2f>,
}
impl Wall
{
    pub fn new(pos : Vector2f) -> Wall 
    {
        let mut texture: SfBox<Texture> = Texture::new().unwrap();
        if texture.load_from_file("./assets/Sprites/Game/wood.png", IntRect::new(0, 0, 0, 0)).is_err()
        {
            panic!("Error on load texture");
        }
        
        Wall
        {
            texture,
            pos,
            dragged    : false,
            selected   : false,
            died       : false,
            color      : None,
            scale      : None,
        }
    }
    pub fn get_sprite(&self) -> Sprite
    {
        let mut sprite : Sprite = Sprite::new();

        sprite.set_position(self.pos);
        sprite.set_texture(&self.texture, true);
        if self.color.is_some()
        {
            sprite.set_color(self.color.unwrap());
        }
        if self.scale.is_some()
        {
            sprite.set_scale(self.scale.unwrap());
        }
        sprite
    }
    pub fn is_coll(&self, ball_shape : CircleShape) -> bool
    {
        let mut collided : bool = false;
        let mut point : Vector2f = Vector2f::new(0.0, 0.0);
        let mut i : f32 = 0.0;

        loop
        {
            point.x = (i.cos() * (ball_shape.radius()+ball_shape.outline_thickness())) + ball_shape.position().x;
            point.y = (i.cos() * (ball_shape.radius()+ball_shape.outline_thickness())) + ball_shape.position().y;
            
            if ball_shape.position().x + ball_shape.radius() + ball_shape.outline_thickness() > self.pos.x 
            && ball_shape.position().x - ball_shape.radius() - ball_shape.outline_thickness() < self.pos.x + self.get_sprite().global_bounds().width
            && ball_shape.position().y - ball_shape.radius() - ball_shape.outline_thickness() < self.pos.y + self.get_sprite().global_bounds().height
            && ball_shape.position().y + ball_shape.radius() + ball_shape.outline_thickness() > self.pos.y
            {
                collided = true;
            }     
            if i == 360.0
            {
                break;
            }
            i += 1.0;
        }

        collided
    }
    pub fn is_coll_wall(&self, rect_shape : &Wall) -> bool
    {
        if self.pos.x + self.texture.size().x as f32 > rect_shape.pos.x
        && self.pos.x < rect_shape.pos.x + rect_shape.texture.size().x as f32
        && self.pos.y < rect_shape.pos.y + rect_shape.texture.size().y as f32
        && self.pos.y + self.texture.size().y as f32 > rect_shape.pos.y
        {
            return true
        }

        false
    }
    pub fn detect_colision(&self, ball_shape : CircleShape, ball_shape_ant : CircleShape) -> u32//0->None 1->LEFT-RIGHT 2->TOP-DOWN
    {
        if self.is_coll(ball_shape.clone())
        && ball_shape.position().x + ball_shape.radius() + ball_shape.outline_thickness() > self.pos.x
        && ball_shape.position().x - ball_shape.radius() - ball_shape.outline_thickness() < self.pos.x + self.get_sprite().global_bounds().width
        {
            if ball_shape.position().y + ball_shape.radius() + ball_shape.outline_thickness()*0.9 < self.pos.y
            || ball_shape_ant.position().y + ball_shape_ant.radius() + ball_shape_ant.outline_thickness()*0.9 < self.pos.y
            || ball_shape.position().y - ball_shape.radius() -  ball_shape.outline_thickness()*0.9 > self.pos.y + self.get_sprite().global_bounds().width
            || ball_shape_ant.position().y - ball_shape_ant.radius() - ball_shape.outline_thickness()*0.9 > self.pos.y + self.get_sprite().global_bounds().height
            {
                return 2;
            }
            else
            {
                return 1;
            }
        }
        0
    }
    pub fn draw(&self, win: &Window)
    {
        win.window.draw_sprite(&self.get_sprite(), &RenderStates::DEFAULT);
    }
    pub fn is_over(&self, x_pos: Vector2f) -> bool
    {
        if x_pos.x > self.get_sprite().global_bounds().left
        && x_pos.x < self.get_sprite().global_bounds().left + self.get_sprite().global_bounds().width
        && x_pos.y > self.get_sprite().global_bounds().top
        && x_pos.y < self.get_sprite().global_bounds().top + self.get_sprite().global_bounds().height
        {
            return true;
        }
        false
    }
}