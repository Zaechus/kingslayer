#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn setup_cli_works_json() {
        let _cli = get_world("data/world.json");
    }

    #[test]
    fn integration_cli_take() {
        let cli = get_world("data/world.json");
        cli.ask("take all");
        assert!(cli.ask("i").contains("iron sword") && !cli.ask("l").contains("iron sword"))
    }
}
