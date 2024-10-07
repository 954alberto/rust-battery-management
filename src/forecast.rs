use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents a single forecast for energy consumption.
#[derive(Deserialize, Serialize)]
pub struct Forecast {
    /// Start time of the forecast period.
    pub start: DateTime<Utc>,

    /// End time of the forecast period.
    pub end: DateTime<Utc>,

    /// Average power consumption during the forecast period in MW.
    pub consumption_average_power_interval: f64,
}

/// A collection of forecasts.
#[derive(Deserialize, Serialize)]
pub struct Forecasts {
    /// A list of individual forecasts.
    pub forecasts: Vec<Forecast>,
}

/// Loads forecasts from a JSON file.
///
/// # Parameters
/// - `file_path`: The path to the JSON file containing the forecasts.
///
/// # Returns
/// A `Result` containing `Forecasts` on success or an error on failure.
///
/// # Errors
/// Returns an error if the file cannot be read or if the JSON data is invalid.
pub fn load_forecasts(file_path: &str) -> Result<Forecasts> {
    // Attempt to read the forecasts file
    let data = fs::read_to_string(file_path)
        .context(format!("Unable to read forecasts file: {}", file_path))?;

    // Log the successful reading of the file
    info!("Successfully read forecasts from file: {}", file_path);

    // Attempt to parse the JSON data
    let forecasts: Forecasts =
        serde_json::from_str(&data).context("JSON parsing error in forecasts")?;

    // Validate the forecasts data
    for forecast in &forecasts.forecasts {
        validate_forecast(forecast)?;
    }

    // Log the successful parsing of the data
    info!("Successfully parsed forecasts data.");

    Ok(forecasts) // Return the parsed forecasts wrapped in Ok
}

/// Validates a forecast for energy consumption.
///
/// # Arguments
///
/// * `forecast`: A reference to the `Forecast` to validate.
///
/// # Returns
/// A `Result` indicating success or failure of the validation.
fn validate_forecast(forecast: &Forecast) -> Result<()> {
    if forecast.consumption_average_power_interval < 0.0 {
        return Err(anyhow!(
            "Consumption average power interval must be non-negative."
        ));
    }
    if forecast.start >= forecast.end {
        return Err(anyhow!("Forecast start time must be before end time."));
    }
    Ok(())
}
