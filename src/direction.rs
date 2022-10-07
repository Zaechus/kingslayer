pub(crate) trait Direction {
    fn is_direction(&self) -> bool;
}

impl Direction for str {
    fn is_direction(&self) -> bool {
        matches!(
            self,
            "north"
                | "south"
                | "east"
                | "west"
                | "northeast"
                | "northwest"
                | "southeast"
                | "southwest"
                | "up"
                | "down"
                | "exit"
        )
    }
}
