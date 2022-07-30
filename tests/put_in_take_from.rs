#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn put_in_take_from() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        cli.ask("take leaf");
        cli.ask("drop the leaf");
        cli.ask("take leaf");
        cli.ask("s");
        assert!(
            cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
        );

        assert!(
            cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
                && cli.ask("l").contains("red block")
        );
        assert_eq!(cli.ask("put leaf in large capsule"), "Placed.");
        assert!(
            cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
                && !cli.ask("i").contains("leaf")
        );
        assert_eq!(cli.ask("i"), "Your inventory is empty.");

        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(cli
            .ask("take leaf from large capsule")
            .contains("is closed"));
        assert_eq!(cli.ask("open capsule"), "Opened.");
        assert_eq!(cli.ask("take leaf from large capsule"), "Taken.");
        assert_eq!(cli.ask("take red block from large capsule"), "Taken.");
        assert!(
            cli.ask("i").contains("leaf")
                && cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("leaf")
        );

        assert_eq!(cli.ask("take all"), "Taken. ");
        assert!(cli.ask("i").contains("leaf") && cli.ask("i").contains("capsule"));

        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(cli.ask("put leaf in large capsule").contains("is closed"));
        assert_eq!(cli.ask("open capsule"), "Opened.");
        assert_eq!(cli.ask("put leaf in large capsule"), "Placed.");
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
