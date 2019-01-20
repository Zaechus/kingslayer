#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn open_path() {
        let cli = get_world("data/world.json");

        assert_eq!(cli.ask("enter door"), "The way is closed.");
        assert_eq!(cli.ask("open door"), "Opened.");
        assert!(cli.ask("enter door").contains("Closet"));
    }
}
