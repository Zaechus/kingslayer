#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn look() {
        let mut game: Game = include_str!("test.ron").parse().unwrap();

        let expected = "Center Room\nYou are in the center room.";
        assert_eq!(game.ask("l"), expected);
        assert_eq!(game.ask("look"), expected);
        assert_eq!(game.ask("look around"), expected);
        assert_eq!(game.ask("look center room"), expected);
        assert_eq!(game.ask("look room"), expected);
        assert_eq!(game.ask("look at room"), expected);
    }

    #[test]
    fn examine() {
        let mut game: Game = include_str!("test.ron").parse().unwrap();

        let expected = "There is nothing remarkable about the Center Room.";
        assert_eq!(game.ask("examine room"), expected);
        assert_eq!(game.ask("examine this room"), expected);
        assert_eq!(game.ask("examine the center room"), expected);
    }
}
