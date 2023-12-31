mod actor;
mod game_constants;
mod game;
mod textgame;
mod ioadapter;
use ioadapter::console::Console;
use textgame::textgame::start_text_game;


fn main() {
    let console = Console;
    start_text_game(&console);
}
