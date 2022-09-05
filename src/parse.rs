use crate::{entity::room::Room, player::Player, tokens::Tokens};

pub(crate) fn parse_take(
    verb: &str,
    command: &Tokens,
    player: &mut Player,
    room: &mut Room,
) -> String {
    if let Some(obj) = command.obj() {
        if let Some(_prep) = command.prep() {
            if let Some(obj_prep) = command.obj_prep() {
                todo!("take item from container")
            } else {
                format!("What do you want to {} from?", verb)
            }
        } else if obj == "all" {
            player.take_all(room.give_all())
        } else {
            player.take(room.give(obj))
        }
    } else {
        format!("What do you want to {}?", verb)
    }
}

pub(crate) fn parse_drop(
    verb: &str,
    command: &Tokens,
    player: &mut Player,
    room: &mut Room,
) -> String {
    if let Some(obj) = command.obj() {
        room.take(player.drop(obj))
    } else {
        format!("What do you want to {}?", verb)
    }
}
