#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn increase_stat_pts() {
        let cli = Cli::from_json_file("data/test_world.json");

        assert!(cli.ask("status").contains("Stat points: 4"));
        assert!(cli
            .ask("increase dex")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 3"));
        assert!(cli.ask("stats").contains("Dexterity: 1"));
        assert!(cli
            .ask("increase charisma")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 2"));
        assert!(cli.ask("stats").contains("Charisma: 1"));
        assert!(cli
            .ask("increase cha")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 1"));
        assert!(cli.ask("stats").contains("Charisma: 2"));
        assert!(cli
            .ask("increase wisdom")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 0"));
        assert!(cli.ask("stats").contains("Wisdom: 1"));
        assert!(cli
            .ask("increase wisdom")
            .contains("You do not have any stat points."));
        assert!(cli.ask("stats").contains(""));
    }
}
