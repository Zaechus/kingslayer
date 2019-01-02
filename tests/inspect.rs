#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn player_inspect() {
        let mut cli = get_world("data/world.json");

        assert!(cli.ask("i").contains("leaf") && !cli.ask("l").contains("leaf"));
        assert_eq!(cli.ask("inspect leaf"), "It's small, brown, and dry.");
        cli.ask("drop leaf");
        assert!(cli.ask("l").contains("leaf") && !cli.ask("i").contains("leaf"));
        assert_eq!(cli.ask("inspect leaf"), "It's small, brown, and dry.");
    }
}
