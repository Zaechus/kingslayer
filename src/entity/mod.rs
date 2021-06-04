mod ally;
mod breakable;
mod closeable;
mod element;
mod enemy;
pub mod item;
mod lockable;
mod pathway;
mod room;

pub use ally::Ally;
pub use breakable::{Breakable, Durability};
pub use closeable::{Closeable, Opening};
pub use element::Element;
pub use enemy::Enemy;
pub use item::Item;
pub use lockable::{DoorLock, Lockable};
pub use pathway::Pathway;
pub use room::Room;

pub trait Entity {
    fn name(&self) -> &str;

    fn desc(&self) -> &str;

    fn inspect(&self) -> &str;
}
