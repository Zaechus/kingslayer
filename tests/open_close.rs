#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn open_close_path() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        assert!(cli.ask("l").contains("way is shut"));
        assert_eq!(cli.ask("enter door"), "The way is shut.");
        assert_eq!(cli.ask("open door"), "Opened.");
        assert!(cli.ask("l").contains("The way is open."));

        assert!(
            cli.ask("enter door").contains("Closet") && cli.ask("l").contains("The way is open.")
        );
        assert_eq!(cli.ask("close door"), "Closed.");
        assert!(cli.ask("l").contains("way is shut"));
        assert_eq!(cli.ask("enter door"), "The way is shut.");
        assert_eq!(cli.ask("open door"), "Opened.");
        assert!(cli.ask("l").contains("The way is open."));

        assert!(cli.ask("enter door").contains("Central Room"));
        assert!(cli.ask("l").contains("The way is open."));
    }

    #[test]
    fn open_close_item() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        cli.ask("s");

        assert!(cli.ask("open capsule").contains("already opened"));
        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(
            cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("red block")
        );
        assert_eq!(cli.ask("open capsule"), "Opened.");
        assert!(
            cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
                && cli.ask("l").contains("red block")
        );
        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(cli.ask("close capsule").contains("already closed"));
        assert!(
            cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("red block")
        );

        cli.ask("take all");
        assert!(
            !cli.ask("l").contains("capsule")
                && cli.ask("i").contains("capsule")
                && !cli.ask("i").contains("contains")
                && !cli.ask("i").contains("red block")
        );
        assert_eq!(cli.ask("open capsule"), "Opened.");
        assert!(cli.ask("open capsule").contains("already opened"));
        assert!(
            cli.ask("i").contains("capsule")
                && cli.ask("i").contains("contains")
                && cli.ask("i").contains("red block")
        );
        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(cli.ask("close capsule").contains("already closed"));
        assert!(
            cli.ask("i").contains("capsule")
                && !cli.ask("i").contains("contains")
                && !cli.ask("i").contains("red block")
        );
    }
}
