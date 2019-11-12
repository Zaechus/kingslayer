#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn player_equip() {
        let cli = Cli::from_json_file("worlds/test_world.json");

        cli.ask("n");
        cli.ask("take iron sword");
        assert!(cli.ask("i").contains("iron sword") && !cli.ask("i").contains("Main hand"));
        assert!(cli.ask("draw iron sword").contains("Equipped."));
        assert!(
            cli.ask("i").contains("Main hand: iron sword")
                && !cli.ask("i").contains("  iron sword")
        );
    }

    #[test]
    fn player_don() {
        let cli = Cli::from_json_file("worlds/test_world.json");

        cli.ask("take leather armor");
        assert!(cli.ask("i").contains("leather armor") && !cli.ask("i").contains("Armor"));
        assert!(cli.ask("don leather armor").contains("Donned."));
        assert!(
            cli.ask("i").contains("Armor: leather armor")
                && !cli.ask("i").contains("  leather armor")
        );
    }
}
