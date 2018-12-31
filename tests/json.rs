#[cfg(test)]
mod tests {
    use kingslayer::get_world;

    #[test]
    fn setup_cli_json() {
        let _cli = get_world("data/skel_world.json");
        let _cli = get_world("data/test_world.json");
        let _cli = get_world("data/world.json");
    }
}
