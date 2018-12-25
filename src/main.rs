fn main() {
    let cli = kingslayer::get_world("data/world.json");

    println!("{}", cli.ask("l"));
    loop {
        match cli.ask(&cli.prompt()).as_str() {
            ref s if s.contains("You died.") => {
                println!("{}", s);
                break;
            }
            ref s => println!("{}", s),
        }
    }
}
