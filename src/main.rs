// use bindings::{
//     windows::win32::system_services::MB_OK,
//     windows::win32::windows_and_messaging::{MessageBoxA, HWND},
// };

mod game;
use game::HangmaneGame;

fn main() {
    let mut game = HangmaneGame::new(String::from("secret"));
    game.play();
}