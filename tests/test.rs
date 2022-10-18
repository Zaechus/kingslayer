#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn test() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        // no input
        game.ask("");

        // no verb or noun
        game.ask("a");

        // and with no clauses
        game.ask("and");
    }

    #[test]
    fn look() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        let expected = "Center Room\nYou are in the center room.\nThere is a box here.";
        for s in [
            "l",           // alias
            "look",        // look command
            "look around", // long form
        ] {
            assert_eq!(game.ask(s), expected);
        }
    }

    #[test]
    fn examine() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        // examining a closed item without a special desc should explain the item is closed
        assert_eq!(game.ask("examine box"), "The box is closed.");

        // examining an open item with contents should return the contents
        game.ask("open box");
        assert_eq!(game.ask("examine box"), "The box contains:\n  a apple");

        // examining an open item with no contents should return that the item is empty
        game.ask("take apple");
        assert_eq!(game.ask("examine box"), "The box is empty.");

        // item with nothing special
        assert_eq!(
            game.ask("examine apple"),
            "There is nothing remarkable about the apple."
        );
    }

    #[test]
    fn names() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        // moving rooms
        assert!(game.ask("l").starts_with("Center Room"));
        assert!(game.ask("enter closet").starts_with("Closet"));

        // name matching

        let expected = "It's here.";

        assert_eq!(game.ask("where sword"), expected);
        for x in [
            "sword",           // 1/2 words
            "iron sword",      // exact match 2 words
            "iron",            // 1/2 words alt
            "block",           // 1/3 words
            "red block",       // 2/3 words
            "large red block", // exact match 3 words
        ] {
            assert_eq!(game.ask(format!("where is the {}", x)), expected)
        }

        for x in [
            "big red block",  // adj big is not in any present item
            "big blue block", // two adj not present
            "plate",          // item not found
        ] {
            assert_ne!(game.ask(format!("where is the {}", x)), expected)
        }
    }

    #[test]
    fn open_close_container() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        // reveal message
        assert_eq!(game.ask("open box"), "Opening the box reveals a apple.");

        // already open
        assert_eq!(game.ask("open box"), "The box is already open.");

        // take object from unspecified open container
        assert_eq!(game.ask("take apple"), "Taken.");

        // close
        assert_eq!(game.ask("close box"), "Closed.");

        // already closed
        assert_eq!(game.ask("close box"), "The box is already closed.");

        // open with no reveal
        assert_eq!(game.ask("open box"), "Opened.");
    }

    #[test]
    fn take_all() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        game.ask("n and take all");
    }
}
