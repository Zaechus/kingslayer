#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn readme_example() {
        let cli = Cli::from_ron_file("worlds/world.ron");

        println!("{}", cli.ask("l"));
        loop {
            let s = cli.ask("");
            println!("{}", s);
            if s.contains("You died.") {
                break;
            }
            break;
        }
    }

    #[test]
    fn from_file() {
        let _cli = Cli::from_json_file("worlds/test_world.json");
        let _cli = Cli::from_ron_file("worlds/test_world.ron");
        let _cli = Cli::from_ron_file("worlds/world.ron");
        let _cli = Cli::from_ron_file("worlds/zork.ron");
    }
}
