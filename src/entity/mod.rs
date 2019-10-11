mod ally;
mod closeable;
mod enemy;
mod item;
mod pathway;
mod room;

pub use ally::Ally;
pub use closeable::Closeable;
pub use enemy::Enemy;
pub use item::Item;
pub use pathway::Pathway;
pub use room::Room;

pub trait Entity {
    fn name(&self) -> &String;

    fn desc(&self) -> &String;

    fn inspect(&self) -> &String;
}
