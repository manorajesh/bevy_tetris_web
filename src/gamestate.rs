use crate::{
    bag::Bag,
    gamescore::GameScore,
    tetlib::{init, new_piece},
    tetrominoe::Tetrominoe,
};
use bevy::prelude::Resource;

// impl GameWrapper {
//     fn verify(&self) -> bool {
//         let mut hasher = DefaultHasher::new();
//         self.game.hash(&mut hasher);
//         let hash = hasher.finish();
//         hash == self.hash
//     }
// }

#[derive(Clone, Resource)]
pub struct GameState {
    pub display: Vec<Vec<Tetrominoe>>,
    pub active_piece: Tetrominoe,
    pub hold_piece: Option<Tetrominoe>,
    pub next_piece: Tetrominoe,
    pub counter: usize,
    pub is_game_over: bool,
    pub bag: Bag,
    pub gamescore: GameScore,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut gs = GameState {
            display: init(width, height),
            active_piece: Tetrominoe::new(None, None),
            hold_piece: None,
            next_piece: Tetrominoe::random(&mut Bag::new()),
            counter: 0,
            is_game_over: false,
            bag: Bag::new(),
            gamescore: GameScore::new(),
        };
        new_piece(&mut gs, None);
        gs
    }
}

// return true if user wants to repeat
//     pub fn serial(&mut self) -> bool {
//         // repeat
//         if confirmation("New game?") {
//             return true;
//         }

//         // saving
//         if !confirmation("Save game?") {
//             return false;
//         }

//         let path = String::from("save.tetris");

//         if Path::new(&path).exists() && !confirmation("Overwrite save?") {
//             return false;
//         }

//         let mut file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .open(&path)
//             .unwrap();

//         self.gamescore.stop_timer();
//         let mut hasher = DefaultHasher::new();
//         self.hash(&mut hasher);
//         let hash = hasher.finish();

//         let game_wrapper = GameWrapper {
//             game: self.clone(),
//             hash,
//         };

//         if !game_wrapper.verify() {
//             println!("Hash verification failed. Aborting save.");
//             return false;
//         }

//         let serialized_data = serialize(&game_wrapper).expect("Failed to serialize game.");
//         file.write_all(&serialized_data).unwrap();
//         false
//     }

//     pub fn deserial(path: String, width: usize, height: usize) -> Self {
//         if !Path::new(&path).exists() {
//             return GameState::new(width, height);
//         }

//         let mut file = OpenOptions::new().read(true).open(path).unwrap();

//         let mut serialized_data = Vec::new();
//         file.read_to_end(&mut serialized_data).unwrap();

//         let game_wrapper: GameWrapper =
//             deserialize(&serialized_data).expect("Failed to deserialize game.");

//         if !game_wrapper.verify() {
//             println!("Save file is corrupted. Starting new game.");
//             sleep(Duration::from_secs(2));
//             return GameState::new(width, height);
//         }

//         let mut game = game_wrapper.game;
//         game.gamescore.reset_timer();
//         init(width, height);
//         game
//     }
// }

// fn confirmation(prompt: &str) -> bool {
//     loop {
//         put_text(
//             WIDTH.try_into().unwrap(),
//             HEIGHT.try_into().unwrap(),
//             format!("{} (y/n)", prompt).as_str(),
//         );
//         loop {
//             let key = get_input();
//             match key {
//                 'y' => return true,
//                 'n' => return false,
//                 _ => continue,
//             }
//         }
//     }
// }

// #[macro_export]
// macro_rules! handle_input {
//     ($x:expr) => {{
//         print!("{}", $x);
//         std::io::stdout().flush().unwrap();
//         let stdin = std::io::stdin();
//         let mut buf = String::new();
//         stdin.read_line(&mut buf).unwrap();
//         buf.trim().to_string()
//     }};
// }
