#[cfg(test)]
mod tests {
    use kingslayer::Game;

    #[test]
    fn basic() {
        Game::from_ron_str(include_str!("test.ron"));
    }
}
