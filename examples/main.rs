extern crate kifuwarabe_movement;
extern crate kifuwarabe_position;

use kifuwarabe_movement::*;
use kifuwarabe_position::*;

fn main()
{
    let _game_record = GameRecord::new();
    let mut position = Kyokumen::new();
    let movement = Movement::from_hash(0);//投了

    let _km = make_movement(&Sengo::Sen, &movement, &mut position);

}