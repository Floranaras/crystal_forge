use crate::game_state::GameState;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum MarketCondition {
    Recession,    // Low prices
    Stable,       // Normal prices  
    Boom,         // High prices
}

impl MarketCondition {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Self::Recession,
            1 => Self::Stable,
            _ => Self::Boom,
        }
    }

    pub fn calculate_price(&self, production_cost: i32) -> i32 {
        let mut rng = rand::thread_rng();
        
        let (min_price, max_price) = match self {
            Self::Recession => (20, production_cost - 10),
            Self::Stable => (80, (production_cost * 105) / 100),
            Self::Boom => (production_cost, production_cost * 4),
        };

        if max_price <= min_price {
            return min_price;
        }

        rng.gen_range(min_price..=max_price)
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Recession => "Market Recession",
            Self::Stable => "Stable Market",
            Self::Boom => "Market Boom",
        }
    }
}

pub fn expire_inventory(game: &mut GameState) {
    if game.inventory_units <= 0 {
        return;
    }
    
    println!("\nInventory Expiration - Crystal units lose potency at cycle end.");
    println!("{} units have expired and been discarded.", game.inventory_units);
    game.inventory_units = 0;
}
