#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn game() {
        let mut game = Game::from_ron_str(include_str!("test.ron"));

        assert!(game.ask("l").contains("Center Room"));
    }
}
