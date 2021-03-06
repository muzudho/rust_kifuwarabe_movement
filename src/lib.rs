extern crate kifuwarabe_position;

use kifuwarabe_position::*;
use std::fmt;


/**********
 * 論理値 *
 **********/
 /**
  * false => 0
  * true => 1
  *
  * bool は i32 だが、_to_num 系は usize を返すように合わせるぜ☆（*＾～＾*）
  */
#[allow(dead_code)]
pub fn bool_to_num(b:bool) -> usize {
    b as usize
}
/**
 * 0 なら偽、それ以外は真☆（＾～＾）
 */
#[allow(dead_code)]
pub fn num_to_bool(n:usize) -> bool {
    match n {
        0 => false,
        _ => true
    }
}
/**
 * ハッシュ値を作る
 */
#[allow(dead_code)]
pub fn push_bool_to_hash(hash:u64, b:bool) -> u64 {
    // bool は i32 だが、hash は u64 なので u64 に合わせるぜ☆（*＾～＾*）
    (hash<<7) + b as u64
}
/**
 * ハッシュ値から作る
 */
#[allow(dead_code)]
pub fn pop_bool_from_hash(hash:u64) -> (u64, bool) {
    let b_num = num_to_bool( (hash & 0b1) as usize );
    (hash>>7, b_num)
}



/********
 * 手目 *
 ********/
/**
 * 手目数。何手目まで指せるか。
 * 棋譜を残す配列のサイズでもある。
 * 大会ルールが 256手として、終端子として投了を１個入れておけるようにする。
 */
pub const TEME_LN :usize = 257;
/**
 * 同一局面何回で千日手
 */
pub const SENNTITE_NUM :i8 = 4;







/// 棋譜
#[derive(Copy)]
pub struct GameRecord{
    /// 手目
    pub teme : usize,
    /// 局面ハッシュ種
    pub ky_hash_seed : PositionHashSeed,
    /// 現局面ハッシュ
    pub ky_hash : [u64; TEME_LN],
    /// 初期局面ハッシュ
    pub ky0_hash : u64,
    /// 取った駒
    pub cap : [Koma; TEME_LN],
    /// 棋譜
    //#[derive(Copy, Clone)]
    pub moves : [Movement; TEME_LN],
}
impl Clone for GameRecord {
    fn clone(&self) -> GameRecord {
        GameRecord {
            teme: self.teme,
            ky_hash_seed: self.ky_hash_seed, // clone() になる。
            ky_hash: self.ky_hash,
            ky0_hash: self.ky0_hash,
            cap: self.cap,
            moves: self.moves,
        }
    }
}
impl GameRecord {
    pub fn default()->GameRecord{
        GameRecord{
            teme : 0,
            ky_hash_seed : PositionHashSeed{
                // 盤上の駒
                km : [[0; Koma::Num as usize]; BAN_SIZE],
                // 持ち駒
                mg : [[0;MG_MAX]; Koma::Num as usize],
                // 先後
                sn : [0; Sengo::Num as usize],
            },
            ky0_hash : 0,
            ky_hash : [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
                0,//257要素
            ],
            /// 取った駒
            cap : [
                // 1行16要素で並べるぜ☆（＾～＾）
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,  Koma::Kara,
                Koma::Kara//257要素
            ],
            moves : [
                // 1行16要素で並べるぜ☆（＾～＾）
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),  Movement::default(),
                Movement::default()//257要素
            ],
        }
    }
    pub fn set_all(&mut self, source: &GameRecord) {
        self.teme = source.teme;
        self.ky_hash_seed.set_all(&source.ky_hash_seed);
        self.ky_hash = source.ky_hash;
        self.ky0_hash = source.ky0_hash;
        self.cap = source.cap;
        self.moves = source.moves;
    }
    pub fn set_teme(&mut self, teme:usize){
        self.teme = teme
    }
    pub fn get_teme(&self) -> usize {
        self.teme
    }
    /// 手番
    pub fn get_teban(&self, jiai:Jiai)->Sengo{
        use kifuwarabe_position::Jiai::*;
        match jiai {
            Ji=>{
                // 手番
                if self.teme%2==0 {
                    Sengo::Sen
                } else {
                    Sengo::Go
                }
            },
            Ai=>{
                // 相手番
                if self.teme%2==0 {
                    Sengo::Go
                } else {
                    Sengo::Sen
                }
            },
            _ =>{ Sengo::Num },
        }
    }
    /// 自分相手
    pub fn get_jiai_by_km(&self, km:Koma ) -> Jiai {
        let (sn,_kms) = km_to_sn_kms( km );
        if sn == self.get_teban(Jiai::Ji) { Jiai::Ji } else { Jiai::Ai }
    }
    /// 棋譜の作成
    pub fn set_movement(&mut self, mv: Movement){
        self.set_sasite_src(mv.source);
        self.set_sasite_drop(mv.drop);
        self.set_sasite_dst(mv.destination);
        self.set_sasite_pro(mv.promotion);
    }
    pub fn set_sasite_src(&mut self, src:umasu){
        self.moves[self.teme].source = src
    }
    pub fn set_sasite_dst(&mut self, dst:umasu){
        self.moves[self.teme].destination = dst
    }
    pub fn set_sasite_pro(&mut self, pro:bool){
        self.moves[self.teme].promotion = pro
    }
    pub fn set_sasite_drop(&mut self, kms:KmSyurui){
        self.moves[self.teme].drop = kms
    }
    pub fn set_ky0_hash(&mut self, hash:u64){
        self.ky0_hash = hash
    }
    pub fn set_ky1_hash(&mut self, hash:u64){
        self.ky_hash[self.teme] = hash
    }
    #[allow(dead_code)]
    pub fn set_cap(&mut self, teme:usize, km:Koma){
        self.cap[ teme ] = km
    }
    pub fn get_sasite(&self) -> Movement {
        self.moves[self.teme]
    }
    #[allow(dead_code)]
    pub fn get_ky_hash(&mut self) -> u64 {
        self.ky_hash[self.teme]
    }
    /*
    pub fn get_position_hash_seed(&self) -> KyHashSeed {
        self.ky_hash_seed
    }
     */

    /// 初期局面ハッシュを作り直す。先後込み。
    pub fn create_ky0_hash(&self, position0: &Position) -> u64 {
        let mut hash : u64;

        hash = position0.create_hash(&self.ky_hash_seed);

        // 手番ハッシュ（後手固定）
        hash ^= self.ky_hash_seed.sn[Sengo::Go as usize];

        hash
    }

    /// 局面ハッシュを作り直す。先後込み。
    pub fn create_ky1_hash(&self, position1: &Position) -> u64 {
        let mut hash : u64;

        // グローバル変数を使う。
        hash = position1.create_hash(&self.ky_hash_seed);

        use kifuwarabe_position::Sengo::*;
        match self.get_teban(Jiai::Ji) {
            Sen => { hash ^= self.ky_hash_seed.sn[Sengo::Sen as usize] },
            Go => { hash ^= self.ky_hash_seed.sn[Sengo::Go as usize] },
            _ => {},
        }

        hash
    }

    /// 入れた指し手の通り指すぜ☆（＾～＾）
    ///
    /// # Arguments.
    ///
    /// * `movement` - 指し手。
    ///
    /// # Returns.
    ///
    /// 0. 取った駒の種類。
    pub fn make_movement2(&mut self, movement: &Movement, position1: &mut Position) -> KmSyurui
    {
        // 取った駒を記録するために、棋譜に入れる☆
        let cap;
        let sn = self.get_teban(Jiai::Ji);

        // グローバル変数を使う。
        let ky_hash;
        {
            cap = make_movement(sn, movement, position1);
            ky_hash = self.create_ky1_hash(&position1);
        }

        let teme: usize = self.teme;
        self.moves[teme] = *movement;
        self.set_cap(teme, cap);

        // 局面ハッシュを作り直す
        self.set_ky1_hash(ky_hash);
        self.teme += 1;

        km_to_kms(cap)
    }

    /// １手戻す。
    ///
    /// # Returns.
    ///
    /// 0. １手戻せたら真。戻せなかったら偽。
    /// 1. 取った駒の種類。
    pub fn unmake_movement2(&mut self, position1: &mut Position) -> (bool, KmSyurui)
    {
        let mut teme: usize = self.teme;

        if 0 < teme {
            teme -= 1;
            self.teme = teme;
            
            // 棋譜から読取、手目も減る
            let cap = self.cap[teme];
            let sn = self.get_teban(Jiai::Ji);
            let ss = self.get_sasite();

            // グローバル変数を使う。
            unmake_movement(sn, &ss, cap, position1);

            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            (true, km_to_kms(cap))
        } else {
            (false, KmSyurui::Kara)
        }
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    pub fn count_same_ky(&self) -> i8 {
        let mut count = 0;

        if self.get_teme() < 1 { return 0; }
        let last_teme = self.get_teme() - 1;
        let new_teme = self.get_teme();
        // g_writeln( &format!( "Ｃount_same_ky last_teme={} new_teme={}", last_teme ,new_teme ) );
        for i_teme in 0..new_teme {
            let t = last_teme - i_teme;
            // g_writeln( &format!( "i_teme={} t={}", i_teme, t ) );
            if self.ky_hash[t] == self.ky_hash[last_teme] {
                count+=1;
            }
        }

        // 初期局面のハッシュ
        if self.ky0_hash == self.ky_hash[last_teme] {
            count+=1;
        }

        count
    }
}





/// # Movement (ムーブメント;指し手)
///
/// * `source` - 移動元升。打った場合は 0。
/// * `destination` - 移動先升。これが 0 なら投了とするぜ☆（＾～＾）
/// * `promotion` - 移動後に成るなら真。
/// * `drop` - 打の場合、打った駒種類。
#[derive(Copy,Clone)]
pub struct Movement{
    pub source : umasu,
    pub destination : umasu,
    pub promotion : bool,
    pub drop : KmSyurui,
}
impl Movement{
    pub fn default()->Movement{
        Movement{
            source: 0,
            destination: 0,
            promotion: false,
            drop: KmSyurui::Kara,
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self){
        self.source = 0;
        self.destination = 0;
        self.promotion = false;
        self.drop = KmSyurui::Kara;
    }

    /**
     * 考えた結果、指し手が考え付いていれば真。
     */
    pub fn exists(&self) -> bool{
        self.destination != MASU_0
    }
}
impl Movement{
    pub fn to_hash(&self)->u64{
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_kms_to_hash(hash, self.drop);
        hash = push_bool_to_hash(hash, self.promotion);
        hash = push_ms_to_hash(hash, self.destination);
        push_ms_to_hash(hash, self.source)
    }
    pub fn from_hash(hash:u64)->Movement{
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash,src2) = pop_ms_from_hash(hash);
        let (hash,dst2) = pop_ms_from_hash(hash);
        let (hash,pro2) = pop_bool_from_hash(hash);
        let (_hash,drop2) = pop_kms_from_hash(hash);
        Movement{
            source: src2,
            destination: dst2,
            promotion: pro2,
            drop: drop2,
        }
    }
}
impl fmt::Display for Movement{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        // 手が何もない、ぐらいの意味だが、
        // その手を指す場合、投了表示
        if !self.exists() { return write!(f,"resign"); }

        // 投了を弾いたあと、診断☆（＾～＾）
        //assert_banjo_ms(self.destination,"Ｓasite Ｄisplay");
        let (dx,dy) = ms_to_suji_dan(self.destination);

        if self.source==SS_SRC_DA {
            use kifuwarabe_position::KmSyurui;
            write!(f, "{}*{}{}{}",
                match self.drop {
                    KmSyurui::K => { "R" },
                    KmSyurui::Z => { "B" },
                    KmSyurui::I => { "G" },
                    KmSyurui::N => { "S" },
                    KmSyurui::U => { "N" },
                    KmSyurui::S => { "L" },
                    KmSyurui::H => { "P" },
                    _  => { "?" },
                },
                dx,
                num_to_lower_case(dy),
                if self.promotion {"+"}else{""}
            )
        } else {
            let (sx,sy) = if self.source==MASU_0 {
                // エラー・データも表示したい
                 (0,0)
            } else {
                //assert_banjo_ms(self.source,"Ｓasite Ｄisplay＜その２＞");
                ms_to_suji_dan(self.source)
            };
            write!(f, "{}{}{}{}{}",
                sx,
                num_to_lower_case(sy),
                dx,
                num_to_lower_case(dy),
                if self.promotion {"+"}else{""}
            )
        }
    }
}
impl fmt::Debug for Movement{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Movement(source:{}, destination:{}, promotion:{}, drop:{})", self.source, self.destination, self.promotion, self.drop)
    }
}






/**
 * 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
 * 手目のカウントが増えたりはしないぜ☆（＾～＾）
 *
 * return : 取った駒
 */
pub fn make_movement(sn:Sengo, ss:&Movement, position: &mut Position) -> Koma {
    // 動かす駒
    let km;
    // 取った駒
    let cap;

    // 打かどうか
    if ss.source==SS_SRC_DA {
        km = sn_kms_to_km( sn, ss.drop );
        if Koma::Num as usize <= km as usize { panic!("Error: km: {}", km as usize); }
        // 自分の持ち駒を減らす
        position.add_mg(km,-1);
    } else {
        // 打で無ければ、元の升の駒を消す。
        if ss.promotion {
            // 成りなら
            km = km_to_prokm( position.get_km_by_ms(ss.source) );
        } else {
            km = position.get_km_by_ms(ss.source);
        }
        position.set_km_by_ms(ss.source, Koma::Kara);
    }

    // 移動先升に駒があるかどうか
    if let Koma::Kara=position.get_km_by_ms(ss.destination) {
        cap = Koma::Kara;
    } else {
        // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
        cap = position.get_km_by_ms(ss.destination);
        if Koma::Kara as usize != cap as usize {
            // 取った駒があるなら。
            let mg = km_to_mg(cap);
            if (mg as usize) < (Koma::Num as usize) {
                // 持ち駒にならない玉を省く。
                if Koma::Num as usize <= mg as usize { panic!("Error: mg: {}, cap: {}.", mg as usize, cap as usize); }
                position.add_mg(mg,1);
            }
        }
    }

    // 移動先升に駒を置く
    position.set_km_by_ms(ss.destination, km);

    cap
}

/**
 * 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
 * 手目のカウントが増えたりはしないぜ☆（＾～＾）
 */
pub fn unmake_movement(sn:Sengo, ss:&Movement, cap:Koma, position: &mut Position){
    // 移動先の駒
    let km;

    // 打かどうか
    if ss.source==SS_SRC_DA {
        km = sn_kms_to_km(sn, ss.drop);
        if Koma::Num as usize <= km as usize { panic!("Error: km: {}", km as usize); }
        // 自分の持ち駒を増やす
        position.add_mg(km,1);

    // 打で無ければ
    } else if ss.promotion {
        // 成ったなら、成る前へ
        km = prokm_to_km( position.get_km_by_ms(ss.destination) );
    } else {
        km = position.get_km_by_ms(ss.destination);
    }

    // 移動先の駒を、取った駒（あるいは空）に戻す
    position.set_km_by_ms(ss.destination, cap);
    if Koma::Kara as usize != cap as usize {
        // 取った駒があるなら、自分の持ち駒を減らす
        let mg = km_to_mg(cap);
        if (mg as usize) < (Koma::Num as usize) {
            // 持ち駒にならない玉を省く。
            if Koma::Num as usize <= mg as usize { panic!("Error: mg: {}, *cap: {}.", mg as usize, cap as usize); }
            position.add_mg(mg,-1);
        }
    }

    // 移動元升に、動かした駒を置く
    position.set_km_by_ms(ss.source, km);
}
