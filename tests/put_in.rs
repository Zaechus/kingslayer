#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn cli_put_in() {
        let mut cli = get_world("data/test_world.json");

        cli.ask("s");
        cli.ask("drop leaf");
        assert!(cli.ask("l").contains("leaf") && cli.ask("l").contains("capsule"));
        cli.ask("take leaf");
        assert!(cli.ask("i").contains("leaf"));

        cli.ask("put leaf in capsule");
        assert!(
            cli.ask("l").contains("leaf")
                && cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
        );
        assert_eq!(cli.ask("i"), "You are empty-handed.");

        assert!(cli.ask("l").contains("leaf") && cli.ask("l").contains("capsule"));
        cli.ask("take all");
        assert!(cli.ask("i").contains("leaf") && cli.ask("i").contains("capsule"));

        cli.ask("put leaf in capsule");
        assert!(
            cli.ask("i").contains("leaf")
                && cli.ask("i").contains("capsule")
                && cli.ask("i").contains("contains")
        );
        assert!(!cli.ask("l").contains("leaf") && !cli.ask("l").contains("capsule"));
    }
}
