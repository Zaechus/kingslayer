#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn player_equip() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        cli.ask("n");
        cli.ask("take iron sword");
        assert!(cli.ask("i").contains("iron sword") && !cli.ask("i").contains("Main hand"));
        assert!(cli.ask("draw iron sword").contains("Equipped."));
        assert!(
            cli.ask("i").contains("Main hand: iron sword")
                && !cli.ask("i").contains("  iron sword")
        );
        assert!(cli.ask("drop sword").contains("Dropped."));
        assert!(
            !cli.ask("i").contains("Main hand")
                && !cli.ask("i").contains("iron sword")
                && cli.ask("l").contains("iron sword")
        );
        cli.ask("take sword");

        // Replace equipped item with a different one
        cli.ask("s");
        assert!(cli.ask("equip sword").contains("Equipped."));
        assert!(cli.ask("i").contains("Main hand: iron sword"));
        cli.ask("take stick");
        assert!(cli.ask("equip stick").contains("Equipped."));
        let i = cli.ask("i");
        assert!(
            i.contains("Main hand: stick")
                && !i.contains("Main hand: iron sword")
                && i.contains("iron sword")
                && !cli.ask("l").contains("iron sword")
        );
    }

    #[test]
    fn player_don() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        cli.ask("n");
        cli.ask("take leather armor");
        assert!(cli.ask("i").contains("leather armor") && !cli.ask("i").contains("Armor"));
        assert!(cli.ask("don leather armor").contains("Donned."));
        assert!(
            cli.ask("i").contains("Armor: leather armor")
                && !cli.ask("i").contains("  leather armor")
        );
    }
}
