extern crate kinggame1d;

fn main() {
    let cli = kinggame1d::get_world("data/world.json");
    cli.start();
}
