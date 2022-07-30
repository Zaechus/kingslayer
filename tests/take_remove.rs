#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn take_remove() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        cli.ask("take leaf");
        assert!(cli.ask("take leaf").contains("is no"));
        cli.ask("n");
        assert_eq!(cli.ask("remove leaf"), "Dropped.");
        assert_eq!(cli.ask("i"), "Your inventory is empty.");
        assert!(cli.ask("drop leaf").contains("do not have"));
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );

        cli.ask("take the sword");
        assert!(
            cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
        );

        cli.ask("take leaf");
        cli.ask("drop the leaf");
        cli.ask("take that leaf");
        assert!(
            cli.ask("i").contains("iron sword")
                && cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("iron sword")
                && !cli.ask("l").contains("leaf")
        );

        cli.ask("drop iron sword");
        cli.ask("drop iron of sword");
        assert!(
            !cli.ask("i").contains("iron sword")
                && !cli.ask("l").contains("leaf")
                && cli.ask("i").contains("leaf")
                && cli.ask("l").contains("iron sword")
        );

        cli.ask("drop leaf");
        assert_eq!(cli.ask("i"), "Your inventory is empty.");
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );
    }

    #[test]
    fn cli_take_all() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        cli.ask("take leaf");
        cli.ask("n");
        cli.ask("drop leaf");
        assert_eq!(cli.ask("i"), "Your inventory is empty.");
        cli.ask("take all");
        assert!(
            cli.ask("i").contains("iron sword")
                && !cli.ask("l").contains("iron sword")
                && cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("leaf")
        );
    }
}
