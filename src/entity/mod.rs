pub(crate) mod item;
mod open;
pub(crate) mod room;

pub(crate) trait Entity {
    fn name(&self) -> &str;

    fn desc(&self) -> &str;

    fn inspect(&self) -> &str;
}
