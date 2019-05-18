#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn player_inspect() {
        let cli = Cli::from_json_file("data/test_world.json");

        cli.ask("take leaf");
        assert!(cli.ask("i").contains("leaf") && !cli.ask("l").contains("leaf"));
        assert_eq!(cli.ask("inspect leaf"), "It's small, brown, and dry.");
        cli.ask("drop leaf");
        assert!(cli.ask("l").contains("leaf") && !cli.ask("i").contains("leaf"));
        assert_eq!(cli.ask("x leaf"), "It's small, brown, and dry.");
    }
}
