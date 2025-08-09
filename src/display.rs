const CRYSTALS_PER_UNIT: i32 = 10;

pub fn show_cycle_header(cycle: i32, day: i32) {
    println!("\n === Business Cycle {} - Day {} ===", cycle, day);
}

pub fn show_status(_cycle: i32, _day: i32, capital: i32, inventory_units: i32) {
    println!("Capital: {} credits     Inventory: {} units", capital, inventory_units);
}

pub fn show_production_info(production_cost: i32) {
    println!("Current production cost: {} credits per crystal.", production_cost);
    println!("Manufacturing cost: {} credits per unit ({} crystals).", 
             production_cost * CRYSTALS_PER_UNIT, CRYSTALS_PER_UNIT);
}
