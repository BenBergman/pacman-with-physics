use rsfml::graphics;


pub trait Object {
    fn select(&mut self);
    fn unselect(&mut self);
    fn update(&mut self);
    fn draw(&self, rw: &mut graphics::RenderWindow);
}
