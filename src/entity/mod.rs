mod ally;
mod closeable;
mod enemy;
pub mod item;
mod pathway;
mod room;

pub use ally::Ally;
pub use closeable::Closeable;
pub use closeable::Opening;
pub use enemy::Enemy;
pub use item::Item;
pub use pathway::Pathway;
pub use room::Room;

pub trait Entity {
    fn name(&self) -> &str;

    fn desc(&self) -> &str;

    fn inspect(&self) -> &str;
}
