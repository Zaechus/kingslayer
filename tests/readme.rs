#[cfg(test)]
mod tests {
    use kingslayer::Cli;

    #[test]
    fn readme_example() {
        let cli = Cli::from_json_file("data/world.json");

        println!("{}", cli.ask("l"));
        loop {
            let s = cli.ask(&String::new());
            println!("{}", s);
            if s.contains("You died.") {
                break;
            }
            break;
        }
    }
}
