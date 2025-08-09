use crate::{game_state::GameState, input, market};

const CRYSTALS_PER_UNIT: i32 = 10;

pub fn sell_crystals(game: &mut GameState) {
    if game.inventory_units <= 0 {
        println!("No crystal units available for sale.");
        return;
    }

    let price_per_crystal = game.current_trend.calculate_price(game.production_cost);
    
    if price_per_crystal < 0 {
        println!("Trading suspended due to market instability.");
        return;
    }

    let sale_price_per_unit = price_per_crystal * CRYSTALS_PER_UNIT;
    show_trading_info(&game.current_trend, price_per_crystal, sale_price_per_unit);

    loop {
        let sale_result = handle_sale_input(game, sale_price_per_unit);
        
        if sale_result.is_ok() {
            break;
        }
    }
}

fn show_trading_info(trend: &market::MarketCondition, price_per_crystal: i32, sale_price_per_unit: i32) {
    println!("Current market condition: {}", trend.description());
    println!("Crystal Exchange is buying crystals for {} credits each.", price_per_crystal);
    println!("You can earn {} credits per unit.", sale_price_per_unit);
}

fn handle_sale_input(game: &mut GameState, sale_price_per_unit: i32) -> Result<(), &'static str> {
    let units = input::get_integer(
        "How many units do you wish to sell? ", 
        0, 
        game.inventory_units
    );
    
    let proceed = input::get_yes_no(&format!(
        "{} units are about to be sold, proceed? (y/n) ", 
        units
    ));
    
    if !proceed {
        println!("Sale canceled.");
        return Ok(()); // Exit successfully
    }
    
    let total_revenue = sale_price_per_unit * units;
    game.capital += total_revenue;
    game.inventory_units -= units;
    
    println!("{} units sold successfully.", units);
    println!("You earned {} credits from this transaction.", total_revenue);
    
    Ok(())
}
