#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn open_path() {
        let cli = Cli::from_json_file("data/world.json");

        assert_eq!(cli.ask("enter door"), "The way is closed.");
        assert_eq!(cli.ask("open door"), "Opened.");
        assert!(cli.ask("enter door").contains("Closet"));
    }
}
