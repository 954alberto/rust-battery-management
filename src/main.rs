use std::env;
use anyhow::{Context, Result}; // Import Result from anyhow
use battery::Battery;
use forecast::load_forecasts;
use log::info;
use planning::plan_battery_usage;
use prices::load_day_ahead_prices; // Import log macros

mod battery;
mod config;
mod forecast;
mod planning;
mod prices;
mod tests;

/// The main entry point for the battery management application.
///
/// This function initializes the logger, loads forecasts and day-ahead prices,
/// initializes the battery, plans the battery usage, and saves the plan to a file.
///
/// # Returns
/// A `Result` which is `Ok(())` if everything runs successfully, or an error if any step fails.
fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    // Initialize the logger
    env_logger::init();

    // Load configuration from config.toml
    let config = config::load_config("config.toml").context("Failed to load config")?;
    info!("Loaded configuration: {:?}", config);

    // Load forecasts from forecasts.json
    let forecasts_data = load_forecasts("forecasts.json").context("Failed to load forecasts")?;
    info!("Loaded forecasts data successfully.");

    // Initialize the battery with the values from the config
    let battery = Battery::new(
        config.settings.capacity,
        config.settings.initial_charge,
        config.settings.max_rate,
        config.settings.efficiency,
    );

    // Load day-ahead prices from day-ahead.json and calculate the average price
    let (prices_data, average_price) =
        load_day_ahead_prices("day-ahead.json").context("Failed to load day-ahead prices")?;
    info!(
        "Loaded day-ahead prices successfully. Average price: {}",
        average_price
    );

    // Generate the charge/discharge plan using the average price
    let plan = plan_battery_usage(
        forecasts_data.forecasts,
        prices_data.prices,
        battery,
        config.settings.grid_limit, // Pass grid_limit here
        average_price,              // Pass the average price calculated
    )
    .context("Failed to plan battery usage")?;

    // Save the plan to an output file
    planning::save_plan(plan, "output_plan.json").context("Failed to save the plan")?;

    println!("Battery planning complete! Check output_plan.json for details.");
    Ok(()) // Return Ok if everything goes well
}
