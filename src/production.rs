use crate::{game_state::GameState, input, display};

const CRYSTALS_PER_UNIT: i32 = 10;

pub fn manufacture_crystals(game: &mut GameState) {
    display::show_production_info(game.production_cost);
    
    loop {
        let production_result = handle_production_input(game);
        
        if production_result.is_ok() {
            break;
        }
    }
}

fn handle_production_input(game: &mut GameState) -> Result<(), &'static str> {
    let units = input::get_integer(
        "How many crystal units do you wish to manufacture this cycle? ", 
        0, 
        i32::MAX
    );
    
    let total_cost = game.production_cost * CRYSTALS_PER_UNIT * units;
    
    if total_cost > game.capital {
        println!("Insufficient capital to manufacture that many units! Try a smaller amount.");
        return Err("Insufficient capital");
    }
    
    let proceed = input::get_yes_no(&format!(
        "{} units will cost {} credits, proceed? (y/n) ", 
        units, 
        total_cost
    ));
    
    if !proceed {
        println!("Manufacturing canceled. Let's try again.");
        return Err("Manufacturing canceled");
    }
    
    game.capital -= total_cost;
    game.inventory_units += units;
    println!("{} crystal units manufactured.", units);
    
    Ok(())
}
