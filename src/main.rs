use std::{env, process};

use kingslayer::{Armor, Cli, Element, Enemy, EnemyStatus, Gold, Item, Thing, Weapon};

fn main() {
    let cli = if let Some(path) = env::args().nth(1) {
        Cli::from_file(&path).unwrap_or_else(|err| {
            eprintln!("Error reading world file: {}", err);
            process::exit(1);
        })
    } else {
        let cli = Cli::from_ron_str(
            r#"
(world:(curr_room:"Brig",rooms:{"Brig":(name:"Brig",desc:"You are in a small wooden room with a wood pillar in the middle. The ground slowly creaks and rocks beneath you.",paths:[(directions:["door","north"],target:"Hold 1",desc:"There is a door on the north side.",inspect:"The door is plain and wooden.",opening:Some(Closed),lock:Some(Locked("")))],enemies:[],allies:[],elements:[],items:[]),"Crows Nest":(name:"Crows Nest",desc:"You are in a crows nest overlooking the entire ship and sea.",paths:[(directions:["mast","platform","down"],target:"Platform",desc:"There is a platform below you on the central mast.",inspect:"The platform can be reached through holds on the mast.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Platform":(name:"Platform",desc:"You stand on a platform several feet above the main deck.",paths:[(directions:["down"],target:"Main Deck",desc:"The main deck is below you.",inspect:"The main deck is several feet below.",opening:None,lock:None),(directions:["crows nest","up"],target:"Crows Nest",desc:"There is a crows nest above you on the central mast.",inspect:"The crows nest can be reached through holds and rigging on the mast.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Cannon Deck 2":(name:"Cannon Deck",desc:"This deck has cannons lining each side.",paths:[(directions:["north"],target:"Cannon Deck 1",desc:"The deck continues to the north.",inspect:"This ship sure has a lot of cannons.",opening:None,lock:None),(directions:["doorway","south"],target:"Empty Room",desc:"There is a doorway to the south.",inspect:"The doorway is completely dark.",opening:None,lock:None),(directions:["opening","down"],target:"Crew Deck 2",desc:"There is an opening below you.",inspect:"You can see hammocks through the opening.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Captains Cabin":(name:"Captains Cabin",desc:"You stand in a large cabin with the captains belongings littering the ground and hanging on the walls.",paths:[(directions:["door","north"],target:"Main Deck",desc:"There is door at the north end of the room.",inspect:"The door is large with a small dim window in the center.",opening:Some(Open),lock:None)],enemies:[],allies:[],elements:[],items:[]),"Hold 1":(name:"Hold",desc:"You are in the middle of a spacious hold. Crates and barrels array the sides.",paths:[(directions:["door","south"],target:"Brig",desc:"There is a door at the south end of the hold.",inspect:"The door is plain and wooden.",opening:Some(Open),lock:None),(directions:["north"],target:"Hold 2",desc:"The hold continues to the north.",inspect:"This end of the hold is too dark to see into.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Crew Deck 1":(name:"Crew Deck",desc:"You are on a deck with dirty hammocks hanging everywhere.",paths:[(directions:["hatch","down"],target:"Hold 2",desc:"There is a hatch on the ground",inspect:"It looks like a metal grate.",opening:Some(Open),lock:None),(directions:["south"],target:"Crew Deck 2",desc:"The deck continues to the south.",inspect:"There is a doorway at the south end of the deck.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Crew Deck 2":(name:"Crew Deck",desc:"You are on a deck with dirty hammocks hanging everywhere.",paths:[(directions:["opening","up"],target:"Cannon Deck 2",desc:"There is an opening above you.",inspect:"You think you might be able to see the light of day.",opening:None,lock:None),(directions:["north"],target:"Crew Deck 1",desc:"The deck continues to the north.",inspect:"There is a hatch at the north end of the deck.",opening:None,lock:None),(directions:["empty doorway","south"],target:"Infirmary",desc:"There is an empty doorway to the south.",inspect:"The doorway flickers with lantern light.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Cannon Deck 1":(name:"Cannon Deck",desc:"This deck has cannons lining each side.",paths:[(directions:["south"],target:"Cannon Deck 2",desc:"The deck continues to the south.",inspect:"This ship sure has a lot of cannons.",opening:None,lock:None),(directions:["hatch","up"],target:"Main Deck",desc:"A hatch above you bring in bright sunlight.",inspect:"The hatch is a double-hinged grate made of old rusty metal.",opening:Some(Closed),lock:None)],enemies:[],allies:[],elements:[],items:[]),"Infirmary":(name:"Infirmary",desc:"You are in a room with a few empty beds against one wall.",paths:[(directions:["doorway","north"],target:"Crew Deck 2",desc:"The doorway leads back into the crew\'s quarters.",inspect:"The doorway flickers with lantern light.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Hold 2":(name:"Hold",desc:"You are at the end of a spacious hold. Crates and barrels array the sides.",paths:[(directions:["hatch","up"],target:"Crew Deck 1",desc:"There is a hatch above you.",inspect:"It looks like a metal grate.",opening:Some(Closed),lock:None),(directions:["south"],target:"Hold 1",desc:"The hold continues to the south.",inspect:"The middle of the hold is lit by a lantern.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Main Deck":(name:"Main Deck",desc:"The vast open sea surrounds the ship you stand on.",paths:[(directions:["hatch","down"],target:"Cannon Deck 1",desc:"There is a hatch below you.",inspect:"The hatch is a double-hinged grate made of old rusty metal.",opening:Some(Open),lock:None),(directions:["mast","platform","up"],target:"Platform",desc:"There is a platform above you on the central mast.",inspect:"The platform can be reached through holds on the mast.",opening:None,lock:None),(directions:["stairs","south"],target:"Sterncastle",desc:"Stairs towards the south lead upwards onto the sterncastle.",inspect:"The stairs are old and dirty.",opening:None,lock:None),(directions:["door"],target:"Captains Cabin",desc:"There is door on the wall beneath the sterncastle of the ship.",inspect:"The door is large with a small dim window in the center.",opening:Some(Closed),lock:None)],enemies:[],allies:[],elements:[],items:[]),"Sterncastle":(name:"Sterncastle",desc:"You are on the sterncastle of the ship. There is another mast in the center and the ships wheel.",paths:[(directions:["stairs","north","down"],target:"Main Deck",desc:"The stairs lead back onto the main deck.",inspect:"The stairs are old and dirty.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[]),"Empty Room":(name:"Empty Room",desc:"The room is dark and empty.",paths:[(directions:["doorway","north"],target:"Cannon Deck 2",desc:"The doorway leads back into the cannon deck.",inspect:"You see many cannons.",opening:None,lock:None)],enemies:[],allies:[],elements:[],items:[])}))
            "#,
        );

        cli.add_item(
            "Brig",
            Item::Weapon(Weapon::new("stick", "It's short but stout.", 4)),
        );
        cli.add_element(
            "Brig",
            Element::new(
                "center middle wooden pillar",
                "A wooden pillar marks the center of the room.",
                "Don't get a splinter!",
            ),
        );

        let root_beer = Element::new(
            "root beer barrel barrels",
            "You can detect the slight scent of root beer.",
            "You can find no way to open the barrels, but there is definitely root beer inside.",
        );
        cli.add_element("Hold 1", root_beer.clone());
        cli.add_element("Hold 2", root_beer);

        // Hold 1
        cli.spawn_enemy(
            "Hold 1",
            Enemy::new_pirate(EnemyStatus::Asleep)
                .with_hp(1)
                .with_ac(0)
                .with_desc("There is a pirate lying in a chair.")
                .with_inspect("He seems to be intently snoring."),
        );

        // Hold 2
        cli.spawn_enemy("Hold 2", Enemy::new_rats(EnemyStatus::Angry));
        cli.add_item(
            "Hold 2",
            Item::Weapon(Weapon::new(
                "sword",
                "It's a basic short sword with a few knicks.",
                6,
            )),
        );
        cli.add_item(
            "Hold 2",
            Item::Armor(Armor::new(
                "leather armor",
                "It's well worn but reliable.",
                11,
            )),
        );

        // Crew Deck
        cli.spawn_enemy(
            "Crew Deck 1",
            Enemy::new_pirate(EnemyStatus::Asleep).with_item(Item::Gold(Gold::new(10))),
        );
        cli.spawn_enemy(
            "Crew Deck 1",
            Enemy::new_pirate(EnemyStatus::Asleep)
                .with_item(Item::Weapon(Weapon::new(
                    "cutlass",
                    "It has thick steel and many notches.",
                    8,
                )))
                .with_item(Item::Gold(Gold::new(10))),
        );

        // Cannon Deck
        cli.spawn_enemy(
            "Cannon Deck 1",
            Enemy::new_pirate(EnemyStatus::Angry).with_item(Item::Gold(Gold::new(10))),
        );

        // Captains Cabin
        cli.spawn_enemy(
        "Captains Cabin",
            Enemy::new(
                "pirate captain",
                "He grins, showing off multiple golden teeth.",
                EnemyStatus::Angry,
            )
            .with_hp(24)
            .with_ac(10)
            .with_xp(750)
            .with_damage(5)
            .with_item(Item::Thing(Thing::new(
                "blue ring",
                "When the ring catches the sunlight, the surface shimmers like the waves of the sea.",
            )))
            .with_item(Item::Gold(Gold::new(50))),
        );

        // Crows Nest
        cli.spawn_enemy(
            "Crows Nest",
            Enemy::new_pirate(EnemyStatus::Asleep).with_item(Item::Gold(Gold::new(20))),
        );

        cli
    };

    cli.start();
}
