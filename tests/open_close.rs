#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn open_close_path() {
        // TODO
    }

    #[test]
    fn open_close_item() {
        let cli = Cli::from_json_file("data/world.json");

        cli.ask("s");

        assert!(
            cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("curious object")
        );
        assert_eq!(cli.ask("open capsule"), "Opened.");
        assert!(cli.ask("open capsule").contains("already opened"));
        assert!(
            cli.ask("l").contains("capsule")
                && cli.ask("l").contains("contains")
                && cli.ask("l").contains("curious object")
        );
        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(cli.ask("close capsule").contains("already closed"));
        assert!(
            cli.ask("l").contains("capsule")
                && !cli.ask("l").contains("contains")
                && !cli.ask("l").contains("curious object")
        );

        cli.ask("take all");
        assert!(
            !cli.ask("l").contains("capsule")
                && cli.ask("i").contains("capsule")
                && !cli.ask("i").contains("contains")
                && !cli.ask("i").contains("curious object")
        );
        assert_eq!(cli.ask("open capsule"), "Opened.");
        assert!(cli.ask("open capsule").contains("already opened"));
        assert!(
            cli.ask("i").contains("capsule")
                && cli.ask("i").contains("contains")
                && cli.ask("i").contains("curious object")
        );
        assert_eq!(cli.ask("close capsule"), "Closed.");
        assert!(cli.ask("close capsule").contains("already closed"));
        assert!(
            cli.ask("i").contains("capsule")
                && !cli.ask("i").contains("contains")
                && !cli.ask("i").contains("curious object")
        );
    }
}
