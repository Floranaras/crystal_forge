# Crystal Forge

A strategic resource management simulation game built in Rust. Players manage crystal manufacturing operations across multiple business cycles, making decisions based on market conditions and production costs to achieve financial goals.

## Overview

Crystal Forge is a turn-based business simulation where players:
- Manufacture crystal units using capital
- Trade crystals in dynamic market conditions
- Navigate through 10 business cycles
- Aim to reach 1,000,000 credits in capital

## Features

- **Dynamic Market System**: Three market conditions (Recession, Stable, Boom) affecting crystal prices
- **Resource Management**: Balance capital, production costs, and inventory
- **Time Pressure**: Inventory expires at the end of each business cycle
- **Strategic Decisions**: Choose when to manufacture and when to sell based on market trends
- **Developer Mode**: Skip ahead or modify starting conditions for testing

## Installation

### Prerequisites
- Rust 1.70.0 or higher
- Cargo (included with Rust)

### Building from Source
```bash
git clone https://github.com/Floranaras/crystal_forge.git
cd crystal_forge
cargo build --release
```

### Running the Game
```bash
cargo run
```

## Gameplay

### Game Structure
- **10 Business Cycles**: Each cycle represents one round of business operations
- **7 Days per Cycle**: Day 1 for manufacturing, Days 2-7 for trading
- **Inventory Expiration**: All crystal units expire at the end of each cycle

### Market Conditions
- **Recession**: Low crystal prices (20 to production_cost-10 credits)
- **Stable**: Normal prices (80 to 105% of production cost)
- **Boom**: High prices (production_cost to 4x production cost)

### Victory Condition
Accumulate 1,000,000 credits before the end of 10 business cycles.

### Game Over Condition
Fall below 800 credits with no inventory remaining.

## Controls

The game uses text-based input:
- Enter numbers for quantities and amounts
- Use 'y'/'n' or 'yes'/'no' for confirmation prompts
- Follow on-screen instructions for each decision

## Code Structure

The project follows clean architecture principles with separated concerns:

```
src/
├── main.rs           # Entry point and game loop
├── game_state.rs     # Core game state management
├── market.rs         # Market conditions and pricing logic
├── production.rs     # Crystal manufacturing system
├── trading.rs        # Crystal selling mechanics
├── input.rs          # Input validation and handling
└── display.rs        # User interface and output formatting
```

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Building Documentation
```bash
cargo doc --open
```

## Technical Details

- **Language**: Rust 2021 Edition
- **Dependencies**: `rand` for random number generation
- **Architecture**: Modular design with clear separation of concerns
- **Memory Safety**: Leverages Rust's ownership system for safe memory management
- **Error Handling**: Comprehensive error handling using Result types

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

This game was developed as a demonstration of clean architecture principles in Rust, showcasing modular design, proper error handling, and safe memory management.
