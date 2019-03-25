#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn put_in_take_from() {
        let cli = Cli::from_json_file("data/world.json");

        cli.ask("take leaf");
        cli.ask("s");
        assert!(
            cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
        );

        assert_eq!(cli.ask("put leaf in capsule"), "Placed.");
        assert!(
            cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
                && !cli.ask("i").contains("leaf")
        );
        assert_eq!(cli.ask("i"), "You are empty-handed.");

        assert_eq!(cli.ask("take leaf from capsule"), "Taken.");
        assert_eq!(cli.ask("take curious object from capsule"), "Taken.");
        assert!(
            cli.ask("i").contains("leaf")
                && cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("leaf")
        );

        assert_eq!(cli.ask("take all"), "Taken. ");
        assert!(cli.ask("i").contains("leaf") && cli.ask("i").contains("capsule"));

        assert_eq!(cli.ask("put leaf in capsule"), "Placed.");
        assert!(
            cli.ask("i").contains("leaf")
                && cli.ask("i").contains("capsule")
                && cli.ask("i").contains("contains")
        );
        assert!(!cli.ask("l").contains("leaf") && !cli.ask("l").contains("capsule"));

        assert_eq!(cli.ask("take leaf from capsule"), "Taken.");
        assert!(cli.ask("i").contains("leaf"));
    }
}
