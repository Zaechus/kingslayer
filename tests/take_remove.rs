#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn take_remove() {
        let cli = Cli::from_json_file("data/world.json");

        cli.ask("take leaf");
        cli.ask("n");
        assert_eq!(cli.ask("remove leaf"), "Dropped.");
        assert_eq!(cli.ask("i"), "Your inventory is empty.");
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );

        cli.ask("n");
        cli.ask("take the iron sword");
        assert!(
            cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
        );

        cli.ask("take leaf");
        cli.ask("take that leaf");
        cli.ask("take that leaf over there");
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
        let cli = Cli::from_json_file("data/world.json");

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
