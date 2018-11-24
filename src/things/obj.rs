/// What kind of object the Obj is
pub enum ObjType {
    Item,
    Container,
}

/// A representation of all types of objects found within the World
pub trait Obj {
    fn name(&self) -> String;
    fn desc(&self) -> String;
    fn objtype(&self) -> ObjType;
}
