/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_movement;
extern crate kifuwarabe_position;

use kifuwarabe_movement::*;
use kifuwarabe_position::*;

fn main()
{
    let _game_record = GameRecord::default();
    let mut position = Position::default();
    let movement = Movement::from_hash(0);//投了

    let _km = make_movement(Sengo::Sen, &movement, &mut position);

}