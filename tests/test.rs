#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn look() {
        let mut game: Game = include_str!("test.ron").parse().unwrap();

        let expected = "Center Room\nYou are in the center room.";
        for x in [
            "l",
            "look",
            "look around",
            "look around the room",
            "look center room",
            "look room",
            "look at room",
        ] {
            assert_eq!(game.ask(x), expected);
        }
    }

    #[test]
    fn examine() {
        let mut game: Game = include_str!("test.ron").parse().unwrap();

        let expected = "There is nothing remarkable about the Center Room.";
        for x in [
            "examine room",
            "examine this room",
            "examine the center room",
            "inspect center room",
        ] {
            assert_eq!(game.ask(x), expected);
        }
    }
}
