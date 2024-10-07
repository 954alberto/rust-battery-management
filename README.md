# Battery Management System

A Rust-based battery management system that utilizes day-ahead electricity prices and energy consumption forecasts to plan efficient charging and discharging of a battery, preventing consumption from exceeding grid limits and optimizing the battery's usage based on electricity prices.

## Table of Contents
1. [Overview](#overview)
2. [Features](#features)
3. [Project Structure](#project-structure)
4. [Installation](#installation)
5. [Usage](#usage)
6. [Configuration](#configuration)
7. [Testing](#testing)


## Overview

This project is a smart battery management system designed to discharge energy from a battery when consumption exceeds a predefined grid limit, and to charge the battery during periods when electricity prices are below the average.

### Key Components:
- **Battery**: Handles charging and discharging of the battery.
- **Forecasts**: Energy consumption forecast data used to plan battery operations.
- **Prices**: Day-ahead electricity prices data used to optimize when to charge the battery.
- **Planning**: Creates a plan for charging and discharging based on the forecast and prices.
  
## Features

- **Automatic Battery Management**: Prevents grid overuse by discharging energy when consumption is above the limit.
- **Price-Sensitive Charging**: Charges the battery when electricity prices are below the average.
- **Configurable Parameters**: Customize battery capacity, initial charge, efficiency, grid limits, etc.
- **Easy Setup**: Load configurations, forecasts, and price data through JSON files.
- **Efficient Performance**: Optimized for real-time processing of data.

## Project Structure

```plaintext
.
├── src
│   ├── battery.rs        # Battery logic (charging/discharging)
│   ├── config.rs         # Configuration management
│   ├── forecast.rs       # Forecast data handling
│   ├── planning.rs       # Planning logic
│   ├── prices.rs         # Day-ahead electricity prices handling
│   ├── tests.rs          # Unit tests
│   ├── main.rs           # Main entry point
├── benchmarks            # Benchmarking tests (optional)
├── config.toml           # Configuration file
├── forecasts.json        # Example forecasts file
├── day-ahead.json        # Example electricity prices file
├── output_plan.json      # Output plan file
└── README.md             # Project documentation
```
## Installation

1. **Prerequisites**: 
   - Ensure you have [Rust installed](https://www.rust-lang.org/tools/install).

- Build the Project:

```bash
cargo build
```
- Run the Project:

```bash
cargo run
```
## Usage

Input Data:

The system relies on two input JSON files:
```text
forecasts.json: Provides 15-minute interval forecasts of average power consumption.
day-ahead.json: Provides day-ahead electricity prices (in hourly intervals).
```
Examples:
```json
// forecasts.json
{
    "forecasts": [
        {
            "start": "2022-12-12T23:00:00Z",
            "end": "2022-12-12T23:15:00Z",
            "consumption_average_power_interval": 4656000.0
        },
        {
            "start": "2022-12-12T23:15:00Z",
            "end": "2022-12-12T23:30:00Z",
            "consumption_average_power_interval": 4528000.0
        }
    ]
}
```

```json
// day-ahead.json
{
    "bidding_zone": "NL",
    "prices": [
        {
            "start": "2022-12-12T23:00:00Z",
            "end": "2022-12-13T00:00:00Z",
            "market_price_currency": "EUR",
            "market_price_per_kwh": 0.3057
        },
        {
            "start": "2022-12-13T00:00:00Z",
            "end": "2022-12-13T01:00:00Z",
            "market_price_currency": "EUR",
            "market_price_per_kwh": 0.28752
        }
    ]
}
```

Running the System: 
- Once the input files are in place and the configuration is set, you can run the system with:

```bash
cargo run
```
The output will be stored in output_plan.json containing the planned battery usage.

## Configuration
The configuration parameters for the battery management system can be set in the config.toml file.

Key parameters include:

```text
capacity: The maximum capacity of the battery (in MWh).
initial_charge: The initial charge of the battery (in MWh).
max_rate: The maximum charging/discharging rate of the battery (in MW).
efficiency: The efficiency of the battery charging/discharging process (as a fraction).
grid_limit: The maximum allowable consumption from the grid (in Wh).
```

Example:

```toml
# config.toml
[settings]
capacity = 3.0 # Maximum capacity in MWh
initial_charge = 1.5 # Initial charge in MWh
max_rate = 1.5 # Max charging/discharging rate in MW
efficiency = 0.90 # Efficiency in charging/discharging
grid_limit = 7800000.0  # Contractual limit of the grid connection in MW
```

## Testing
To run the unit tests, use the following command:

```bash
cargo test
```
This will execute all tests in the project, ensuring that the functionality works as expected.
