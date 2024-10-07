use crate::battery::Battery;
use crate::forecast::Forecast;
use crate::prices::ElectricityPrice;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc}; // Import DateTime<Utc>
use log::{debug, info}; // Import log macros
use serde::Serialize;
use std::fs;

/// Represents a planned battery usage interval.
#[derive(Serialize)]
pub struct Plan {
    /// Start time of the battery operation.
    pub start: DateTime<Utc>,
    /// End time of the battery operation.
    pub end: DateTime<Utc>,
    /// Energy discharged from the battery in watt-hours.
    pub energy_from_battery_wh: f64,
    /// Energy charged into the battery in watt-hours.
    pub energy_to_battery_wh: f64,
}

/// Plans the battery usage based on forecasts and electricity prices.
///
/// This function checks the forecasts for energy consumption and the prices for
/// charging the battery. If the consumption exceeds the grid limit, it discharges
/// the battery; if the price is low, it charges the battery.
///
/// # Arguments
///
/// * `forecasts`: A vector of forecasted energy consumption data.
/// * `prices`: A vector of day-ahead electricity prices.
/// * `battery`: A mutable reference to the battery being used for charging/discharging.
/// * `grid_limit`: The maximum allowable energy consumption from the grid.
///
/// # Returns
/// A `Result` containing a vector of `Plan` structs if successful, or an error if any step fails.

pub fn plan_battery_usage(
    forecasts: Vec<Forecast>,
    prices: Vec<ElectricityPrice>,
    mut battery: Battery,
    grid_limit: f64,    // Fixed grid limit of 7.8 MW
    average_price: f64, // Average day-ahead price
) -> Result<Vec<Plan>, anyhow::Error> {
    let mut plan = Vec::new();

    for (forecast, price) in forecasts.iter().zip(prices.iter()) {
        let duration_hours = 15.0 / 60.0; // Duration in hours

        debug!(
            "{} - {}",
            forecast.consumption_average_power_interval, grid_limit
        );

        // Check if the consumption exceeds the grid limit
        if forecast.consumption_average_power_interval > grid_limit {
            info!(
                "Consumption of {} exeeds the grid limit {}",
                forecast.consumption_average_power_interval, grid_limit
            );

            let excess = forecast.consumption_average_power_interval - grid_limit;
            debug!("EXCESS: {}", excess);
            // Calculate energy to discharge to meet the grid limit
            let discharged_energy = battery
                .discharge_battery(excess, duration_hours)
                .context("Failed to calculage discharged energy")?; // Handle discharge errors

            info!(
                "Discharging battery: {} Wh at {}",
                (discharged_energy * 1_000_000.0).floor() / 10.0,
                forecast.start
            );

            plan.push(Plan {
                start: forecast.start,
                end: forecast.end,
                energy_from_battery_wh: (discharged_energy * 1_000_000.0).floor() / 10.0, // Energy used from the battery
                energy_to_battery_wh: 0.0, // No energy charged
            });
        } else {
            // If consumption is below the grid limit, check if we can charge the battery
            if price.market_price_per_kwh <= average_price {
                // Using average price directly

                let charge_amount = battery
                    .charge_battery(1.5, duration_hours)
                    .context("Failed to charge battery")?; // Handle charge errors

                info!(
                    "Charging battery: {} Wh at {} (Price: {} EUR/kWh)",
                    (charge_amount * 1_000_000.0).floor() / 10.0,
                    forecast.start,
                    price.market_price_per_kwh
                );

                plan.push(Plan {
                    start: forecast.start,
                    end: forecast.end,
                    energy_from_battery_wh: 0.0, // No energy used from the battery
                    energy_to_battery_wh: (charge_amount * 1_000_000.0).floor() / 10.0, // Energy charged to the battery
                });
            } else {
                // No action needed if price is not favorable for charging
                plan.push(Plan {
                    start: forecast.start,
                    end: forecast.end,
                    energy_from_battery_wh: 0.0,
                    energy_to_battery_wh: 0.0,
                });
            }
        }
    }

    Ok(plan) // Return the plan wrapped in Ok
}

/// Saves the generated battery usage plan to a specified file.
///
/// # Arguments
///
/// * `plan`: A vector of `Plan` structs representing the battery usage plan.
/// * `file_path`: The path to the file where the plan will be saved.
///
/// # Returns
/// A `Result` indicating success or failure of the save operation.
pub fn save_plan(plan: Vec<Plan>, file_path: &str) -> Result<(), anyhow::Error> {
    let planning = serde_json::json!( {
        "planning": plan
    });

    let pretty_output =
        serde_json::to_string_pretty(&planning).context("Error generating pretty JSON")?; // Handle JSON generation errors

    fs::write(file_path, pretty_output)
        .context(format!("Unable to write plan to file: {}", file_path))?; // Handle file write errors

    info!("Saved planning to {}", file_path); // Log saving success
    Ok(()) // Indicate success
}
