#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn increase_stat_pts() {
        let cli = Cli::from_file("worlds/test_world.ron").unwrap();

        assert!(cli.ask("c").contains("Stat points: 4"));
        assert!(cli
            .ask("increase dex")
            .contains("Ability score increased by one."));
        assert!(cli.ask("c").contains("Stat points: 3"));
        assert!(cli.ask("stats").contains("Dexterity:    14 "));
        assert!(cli
            .ask("increase charisma")
            .contains("Ability score increased by one."));
        assert!(cli.ask("c").contains("Stat points: 2"));
        assert!(cli.ask("c").contains("Charisma:     9 "));
        assert!(cli
            .ask("increase cha")
            .contains("Ability score increased by one."));
        assert!(cli.ask("c").contains("Stat points: 1"));
        assert!(cli.ask("c").contains("Charisma:     10 "));
        assert!(cli
            .ask("increase wisdom")
            .contains("Ability score increased by one."));
        assert!(cli.ask("c").contains("Stat points: 0"));
        assert!(cli.ask("c").contains("Wisdom:       11 "));
        assert!(cli
            .ask("increase wisdom")
            .contains("You do not have any stat points."));
    }
}
