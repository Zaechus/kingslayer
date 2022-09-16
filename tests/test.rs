#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn look() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

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
            assert!(game.ask(x).starts_with(expected));
        }
    }

    #[test]
    fn examine() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

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

    #[test]
    fn take() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        assert!(!game.ask("i").contains("box"));
        assert!(!game.ask("i").contains("apple"));
        assert!(game.ask("l").contains("box"));
        assert!(!game.ask("l").contains("apple"));

        assert_eq!(game.ask("open the box"), "Opening the box reveals a apple.");

        assert!(game.ask("l").contains("box"));
        assert!(game.ask("l").contains("apple"));
        assert_ne!(game.ask("open the apple"), "Opened.");

        game.ask("take apple");
        assert!(game.ask("l").contains("box"));
        assert!(!game.ask("l").contains("apple"));

        game.ask("take box");
        assert!(!game.ask("l").contains("box"));
        assert!(!game.ask("l").contains("apple"));
        assert!(game.ask("i").contains("box"));
        assert!(game.ask("i").contains("apple"));
    }
}
