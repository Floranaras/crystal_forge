mod game_state;
mod market;
mod production;
mod trading;
mod input;
mod display;

use game_state::GameState;

fn main() {
    let mut game = GameState::new();
    game.setup_developer_mode();

    while !game.is_finished() {
        if game.is_game_over() {
            println!("Game over! Insufficient capital to continue operations.");
            break;
        }
        
        game.process_business_cycle();
    }

    game.display_final_result();
}
