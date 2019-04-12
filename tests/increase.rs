#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn increase() {
        let cli = Cli::from_json_file("data/world.json");

        assert!(cli.ask("status").contains("Stat points: 4"));
        assert!(cli
            .ask("increase dex")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 3"));
        assert!(cli
            .ask("increase charisma")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 2"));
        assert!(cli
            .ask("increase intel")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 1"));
        assert!(cli
            .ask("increase wisdom")
            .contains("Ability modifier increased by one."));
        assert!(cli.ask("status").contains("Stat points: 0"));
        assert!(cli
            .ask("increase wisdom")
            .contains("You do not have any stat points."));
    }
}
