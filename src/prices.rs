use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Duration, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::fs;

/// Represents the price of electricity for a specific time interval.
#[derive(Deserialize, Serialize)]
pub struct ElectricityPrice {
    /// Start time of the electricity price interval.
    pub start: DateTime<Utc>,

    /// End time of the electricity price interval.
    pub end: DateTime<Utc>,

    /// Currency of the electricity price (e.g., "EUR").
    pub market_price_currency: String,

    /// Price of electricity per kWh.
    pub market_price_per_kwh: f64,
}

/// Represents the day-ahead prices of electricity, containing multiple price intervals.
#[derive(Deserialize, Serialize)]
pub struct DayAheadPrices {
    /// A list of electricity prices for specific time intervals.
    pub prices: Vec<ElectricityPrice>,
}

/// Loads day-ahead electricity prices from a specified JSON file and converts them to 15-minute intervals.
///
/// # Arguments
///
/// * `file_path`: The path to the JSON file containing day-ahead prices.
///
/// # Returns
/// A `Result` containing a `DayAheadPrices` struct if successful, and the average price, or an error if loading or parsing fails.
pub fn load_day_ahead_prices(file_path: &str) -> Result<(DayAheadPrices, f64)> {
    // Attempt to read the day-ahead prices file
    let data = fs::read_to_string(file_path).context(format!(
        "Unable to read day-ahead prices file: {}",
        file_path
    ))?;

    // Attempt to parse the JSON data into DayAheadPrices
    let prices: DayAheadPrices =
        serde_json::from_str(&data).context("JSON parsing error in day-ahead prices")?;

    // Validate the prices data
    for price in &prices.prices {
        validate_price(price)?; // Ensure prices are valid
    }

    // Convert hourly prices into 15-minute intervals
    let fifteen_minute_prices = convert_to_fifteen_minute_intervals(prices.prices);

    // Calculate the average price
    let average_price = fifteen_minute_prices
        .iter()
        .map(|price| price.market_price_per_kwh)
        .sum::<f64>()
        / fifteen_minute_prices.len() as f64;

    info!("Successfully converted hourly prices into 15-minute intervals and loaded day-ahead prices from {}", file_path);

    Ok((
        DayAheadPrices {
            prices: fifteen_minute_prices,
        },
        average_price,
    )) // Wrap the result in Ok
}

/// Converts hourly electricity prices into 15-minute intervals.
/// Each hourly interval is split into four 15-minute intervals with the same price.
///
/// # Arguments
///
/// * `hourly_prices`: A vector of `ElectricityPrice` structs representing hourly prices.
///
/// # Returns
/// A vector of `ElectricityPrice` structs with 15-minute intervals.
fn convert_to_fifteen_minute_intervals(
    hourly_prices: Vec<ElectricityPrice>,
) -> Vec<ElectricityPrice> {
    let mut fifteen_minute_prices = Vec::new();

    for price in hourly_prices {
        let start_time = price.start;
        let price_per_kwh = price.market_price_per_kwh;
        let currency = price.market_price_currency.clone();

        for i in 0..4 {
            let interval_start = start_time + Duration::minutes(i * 15);
            let interval_end = interval_start + Duration::minutes(15);

            fifteen_minute_prices.push(ElectricityPrice {
                start: interval_start,
                end: interval_end,
                market_price_currency: currency.clone(),
                market_price_per_kwh: price_per_kwh,
            });
        }
    }

    fifteen_minute_prices
}

/// Validates an electricity price entry.
///
/// # Arguments
///
/// * `price`: A reference to the `ElectricityPrice` to validate.
///
/// # Returns
/// A `Result` indicating success or failure of the validation.
fn validate_price(price: &ElectricityPrice) -> Result<()> {
    if price.market_price_per_kwh < 0.0 {
        return Err(anyhow!("Market price per kWh must be non-negative."));
    }
    if price.start >= price.end {
        return Err(anyhow!("Price start time must be before end time."));
    }
    Ok(())
}
