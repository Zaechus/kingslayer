#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn setup_cli_json() {
        let _cli = Cli::from_json_file("data/world.json");
    }
}
