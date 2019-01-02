#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn take_remove() {
        let mut cli = get_world("data/world.json");

        assert_eq!(cli.ask("remove leaf"), "Dropped.".to_string());
        assert_eq!(cli.ask("i"), "You are empty-handed.");
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );

        cli.ask("take the iron sword");
        cli.ask("take iron sword");
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
        assert_eq!(cli.ask("i"), "You are empty-handed.");
        assert!(
            cli.ask("l").contains("iron sword")
                && cli.ask("l").contains("leaf")
                && !cli.ask("i").contains("iron sword")
                && !cli.ask("i").contains("leaf")
        );
    }

    #[test]
    fn cli_take_all() {
        let mut cli = get_world("data/world.json");

        cli.ask("drop leaf");
        assert_eq!(cli.ask("i"), "You are empty-handed.");
        cli.ask("take all");
        assert!(
            cli.ask("i").contains("iron sword")
                && !cli.ask("l").contains("iron sword")
                && cli.ask("i").contains("leaf")
                && !cli.ask("l").contains("leaf")
        );
    }
}
