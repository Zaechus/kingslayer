#[cfg(test)]
mod tests {
    use kinggame1d::get_world;

    #[test]
    fn setup_cli_works_json() {
        let _cli = get_world("data/world.json");
    }
}
