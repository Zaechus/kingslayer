#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn test() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        let excuse_me = ["", "a", "and", "a and", "and a"];
        for s in excuse_me {
            assert_eq!(game.ask(s), "Excuse me?");
        }

        // unknown verb
        assert_eq!(game.ask("jump"), "I do not know the verb \"jump\".")
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
    fn examine() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        // examining a closed item without a special desc should explain the item is closed
        assert_eq!(game.ask("examine box"), "The box is closed.");

        // examining an open item with contents should return the contents
        assert_eq!(game.ask("open box"), "Opening the box reveals a apple.");
        assert_eq!(game.ask("examine box"), "The box contains:\n  a apple");

        // examining an open item with no contents should return that the item is empty
        assert_eq!(game.ask("take apple"), "Taken.");
        assert_eq!(game.ask("examine box"), "The box is empty.");

        // item with nothing special
        assert_eq!(
            game.ask("examine apple"),
            "There is nothing remarkable about the apple."
        );
    }

    #[test]
    fn containers() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        // reveal message
        assert_eq!(game.ask("open box"), "Opening the box reveals a apple.");

        // already open
        assert_eq!(game.ask("open box"), "The box is already open.");

        // take object from unspecified open container
        assert_eq!(game.ask("take apple"), "Taken.");

        // try to put item inside itself
        assert_eq!(game.ask("put apple in apple"), "Impossible.");
        assert_eq!(game.ask("put box in box"), "Impossible.");

        // close
        assert_eq!(game.ask("close box"), "Closed.");

        // already closed
        assert_eq!(game.ask("close box"), "The box is already closed.");

        // open with no reveal
        assert_eq!(game.ask("open box"), "Opened.");

        // trying to put an item into a closed container
        assert_eq!(game.ask("close box"), "Closed.");
        assert_eq!(game.ask("put apple in box"), "The box isn't open.");

        // try to put an item that is not in inventory
        assert_eq!(game.ask("drop apple"), "Dropped.");
        assert_eq!(game.ask("put apple in box"), "You do not have the apple.");

        // put item in container
        assert_eq!(game.ask("open box"), "Opened.");
        assert_eq!(game.ask("take apple"), "Taken.");
        assert_eq!(game.ask("put apple in box"), "Done.");
        assert_eq!(game.ask("inventory"), "Your inventory is empty.");
        assert_eq!(game.ask("examine box"), "The box contains:\n  a apple");
        assert_eq!(game.ask("put apple in box"), "You do not have the apple.");
    }

    #[test]
    fn do_all() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        let res = game.ask("n and take all");
        assert!(["large red block: Taken.", "iron sword: Taken."]
            .iter()
            .all(|p| res.contains(p)))
    }

    #[test]
    fn attack() {
        let mut game: Game = include_str!("world.ron").parse().unwrap();

        game.ask("enter arena and take spear");
        assert_eq!(
            game.ask("kill goblin with spear"),
            "You hit the goblin with your spear.\n\nThe goblin hits you."
        );
        assert_eq!(
            game.ask("kill goblin with spear"),
            "You hit the goblin with your spear. It dies."
        );
    }
}
