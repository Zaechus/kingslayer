# kinggame1d
A text-based adventure written in Rust

### Dependencies
* rustc >= 1.30.1
* cargo >= 1.30.0

### Game Actions
* `quit` or `q`: asks if the user would like to quit the game
* `l` or `look`: look around the room; displays a description of the current room
* `n`, `s`, `e`, `w`, `ne`, `nw`, `se`, `sw`, `u`, `d`: different possible directions one can move to access other rooms
* `enter {path}`: enter another room through a specified path
* `i` or `inventory`: display items in your inventory
* `take {item}` or `grab`: take an item from the room
* `drop {item}`: drop an item from your inventory and into the current room
* `take all`: take all items from the room
* `drop all`: drop all items in inventory into the room
* `put {item} in {item}` or `place {item} in {item}`: places an item from your inventory into a container item in the room or your inventory
* `inspect {item}` or `examine {item}`: display special properties of an item