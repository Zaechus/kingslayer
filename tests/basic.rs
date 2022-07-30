#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn readme_example() {
        let cli = Cli::from_file("worlds/world.ron").unwrap();

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
        let _cli = Cli::from_file("worlds/test_world.ron");
        let _cli = Cli::from_file("worlds/world.ron");
    }
}
