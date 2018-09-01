/// An enum type to represent all types of objects present in a Room
pub enum Obj {
    Container(Container),
    Item(Item),
    Weapon(Weapon),
}

/// An Obj able to hold other Objs
pub struct Container {}

/// A generic Obj
pub struct Item {}

/// An Obj with the use of attacking
pub struct Weapon {}
