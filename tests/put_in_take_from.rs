#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn put_in_take_from() {
        let mut cli = get_world("data/world.json");

        cli.ask("s");
        assert!(
            cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
        );

        assert_eq!(cli.ask("put leaf in capsule"), "Placed.".to_string());
        assert!(
            cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
                && !cli.ask("i").contains("leaf")
        );
        assert_eq!(cli.ask("i"), "You are empty-handed.");

        assert_eq!(cli.ask("take leaf from capsule"), "Taken.".to_string());
        assert_eq!(
            cli.ask("take curious object from capsule"),
            "Taken.".to_string()
        );
        assert!(
            cli.ask("i").contains("leaf")
                && cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("leaf")
        );

        assert_eq!(cli.ask("take all"), "Taken.".to_string());
        assert!(cli.ask("i").contains("leaf") && cli.ask("i").contains("capsule"));

        assert_eq!(cli.ask("put leaf in capsule"), "Placed.".to_string());
        assert!(
            cli.ask("i").contains("leaf")
                && cli.ask("i").contains("capsule")
                && cli.ask("i").contains("contains")
        );
        assert!(!cli.ask("l").contains("leaf") && !cli.ask("l").contains("capsule"));

        assert_eq!(cli.ask("take leaf from capsule"), "Taken.".to_string());
        assert!(cli.ask("i").contains("leaf"));
    }
}
