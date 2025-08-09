use crate::{input, display, market, production, trading};
use rand::Rng;

const TARGET_CAPITAL: i32 = 1_000_000;
const MIN_CAPITAL_TO_CONTINUE: i32 = 800;
const MAX_CYCLES: i32 = 10;
const DAYS_PER_CYCLE: i32 = 7;

#[derive(Debug, Clone)]
pub struct GameState {
    pub capital: i32,
    pub inventory_units: i32,
    pub cycle: i32,
    pub production_cost: i32,
    pub current_trend: market::MarketCondition,
    reached_goal: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            capital: 10_000,
            inventory_units: 0,
            cycle: 1,
            production_cost: 0,
            current_trend: market::MarketCondition::Stable,
            reached_goal: false,
        }
    }

    pub fn setup_developer_mode(&mut self) {
        let use_dev_mode = input::get_yes_no("Enable developer mode? (y/n): ");
        
        if !use_dev_mode {
            return;
        }
        
        self.capital = input::get_integer("Enter starting capital: ", 0, i32::MAX);
        self.cycle = input::get_integer("Enter starting cycle (1-10): ", 1, 10);
    }

    pub fn is_finished(&self) -> bool {
        self.cycle > MAX_CYCLES || self.reached_goal
    }

    pub fn is_game_over(&self) -> bool {
        self.capital < MIN_CAPITAL_TO_CONTINUE && self.inventory_units == 0
    }

    pub fn process_business_cycle(&mut self) {
        self.process_cycle_start();
        self.process_cycle_days();
        self.process_cycle_end();
    }

    fn process_cycle_start(&mut self) {
        display::show_cycle_header(self.cycle, 1);
        display::show_status(self.cycle, 1, self.capital, self.inventory_units);
        
        let mut rng = rand::thread_rng();
        self.production_cost = rng.gen_range(80..=120);
        production::manufacture_crystals(self);
        
        self.current_trend = market::MarketCondition::random();
    }

    fn process_cycle_days(&mut self) {
        for day in 2..=DAYS_PER_CYCLE {
            display::show_cycle_header(self.cycle, day);
            display::show_status(self.cycle, day, self.capital, self.inventory_units);
            
            trading::sell_crystals(self);
            
            if self.capital >= TARGET_CAPITAL {
                self.reached_goal = true;
            }
        }
    }

    fn process_cycle_end(&mut self) {
        market::expire_inventory(self);
        self.cycle += 1;
    }

    pub fn display_final_result(&self) {
        if self.reached_goal {
            println!("Congratulations! You've achieved the target capital of 1,000,000 credits!");
        }
        println!("Game finished. Final capital: {} credits", self.capital);
    }
}
