#[cfg(test)]
mod tests {

    use crate::battery::Battery;
    use crate::config;
    use crate::forecast::load_forecasts;
    use crate::prices::load_day_ahead_prices;
    use std::fs;
    use tempfile::NamedTempFile;

    /// Initializes a Battery instance using values from the configuration file.
    ///
    /// # Returns
    /// A Battery instance initialized with the configuration settings.
    fn initialize_battery() -> Battery {
        let config = config::load_config("config.toml").expect("Failed to load configuration");

        Battery::new(
            config.settings.capacity,
            config.settings.initial_charge,
            config.settings.max_rate,
            config.settings.efficiency,
        )
    }

    #[test]
    fn test_battery_charge() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        let energy_charged = battery.charge_battery(1.0, 1.0).unwrap(); // 1 MW for 1 hour

        // Verify the battery charge is as expected (2.4 MWh)
        assert_eq!(battery.charge, 2.4); // Expect 2.4 MWh with 90% efficiency

        // Verify that the energy charged matches the expected energy considering efficiency
        assert_eq!(energy_charged, 0.9); // Expect 0.9 MWh to be stored
    }

    #[test]
    fn test_battery_discharge() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        let energy_charged = battery.charge_battery(1.0, 1.0).unwrap(); // Charge with 1 MW for 1 hour
        println!(
            "After charging: Battery charge is {} MWh, energy charged: {} MWh",
            battery.charge, energy_charged
        );

        let energy_discharged = battery.discharge_battery(1.0, 1.0).unwrap(); // 1 MW for 1 hour
        println!(
            "After discharging: Battery charge is {} MWh, energy discharged: {} MWh",
            battery.charge, energy_discharged
        );

        assert!(
            (battery.charge - 1.289).abs() < 0.001,
            "Expected charge: 1.289 MWh, Actual charge: {}",
            battery.charge
        );
        assert!(
            (energy_discharged - 1.111).abs() < 0.001,
            "Expected discharged: 1.111 MWh, Actual discharged: {}",
            energy_discharged
        );
    }

    #[test]
    fn test_charge_exceed_capacity() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        let energy_charged = battery.charge_battery(5.0, 1.0).unwrap(); // 5 MW for 1 hour
        println!(
            "After charging: Battery charge is {} MWh, energy charged: {} MWh",
            battery.charge, energy_charged
        );

        assert!(
            battery.charge <= 3.0,
            "Battery charge exceeded capacity: {} MWh",
            battery.charge
        );

        assert!(
            (energy_charged - 1.35).abs() < 0.01,
            "Expected charged energy to be 1.35 MWh due to capacity limit, Actual: {}",
            energy_charged
        );
    }

    #[test]
    fn test_discharge_exceed_capacity() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        let energy_discharged = battery.discharge_battery(3.0, 1.0).unwrap(); // Attempt to discharge more than available
        assert!(
            battery.charge.abs() < 0.01,
            "Expected charge: 0.0 MWh, Actual charge: {}",
            battery.charge
        );
        assert!(
            (energy_discharged - 1.5).abs() < 0.01,
            "Expected discharged energy: 1.5 MWh, Actual discharged energy: {}",
            energy_discharged
        );
    }

    #[test]
    fn test_charge_and_discharge_cycle() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery

        battery.charge_battery(1.0, 1.0).unwrap(); // 1 MW for 1 hour
        assert_eq!(battery.charge, 2.4); // Battery charge should be 2.4 MWh

        let discharged_energy = battery.discharge_battery(1.0, 1.0).unwrap(); // 1 MW for 1 hour

        assert!(
            (battery.charge - 1.2889).abs() < 0.0001,
            "Expected charge: 1.2889, Actual charge: {}",
            battery.charge
        );
        assert!(
            (discharged_energy - 1.111).abs() < 0.001,
            "Expected discharged: 1.111, Actual discharged: {}",
            discharged_energy
        );
    }

    #[test]
    fn test_charge_with_efficiency() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        battery.charge_battery(1.0, 1.0).unwrap(); // 1 MW for 1 hour
        assert_eq!(battery.charge, 2.4); // Expect 2.4 MWh with 90% efficiency
    }

    #[test]
    fn test_discharge_with_efficiency() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        battery.discharge_battery(1.0, 1.0).unwrap(); // 1 MW for 1 hour
        assert!(
            (battery.charge - 0.39).abs() < 0.01,
            "Expected charge: 0.39 MWh, Actual charge: {}",
            battery.charge
        );
    }

    #[test]
    fn test_charge_negative_energy() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        battery.charge = 1.5; // Set initial charge to 1.5 MWh
        let result = battery.charge_battery(-1.0, 1.0); // Negative charge attempt

        // Check that the operation returns an Err
        assert!(
            result.is_err(),
            "Expected an error when charging with negative power."
        );

        // Check that the charge remains unchanged
        assert_eq!(battery.charge, 1.5); // Charge should remain unchanged
    }

    #[test]
    fn test_discharge_negative_energy() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        let result = battery.discharge_battery(-1.0, 1.0); // Negative discharge attempt

        // Check that the operation returns an Err
        assert!(
            result.is_err(),
            "Expected an error when discharging with negative power."
        );

        // Check that the charge remains unchanged
        assert_eq!(battery.charge, 1.5); // Charge should remain unchanged
    }

    #[test]
    fn test_full_cycle() {
        let mut battery = initialize_battery(); // Use the new function to initialize the battery
        let energy_charged = battery.charge_battery(1.5, 2.0).unwrap(); // 1.5 MW for 2 hours
        assert_eq!(
            battery.charge, 3.0,
            "Expected charge: 3.0 MWh, Actual charge: {}",
            battery.charge
        ); // Should reach capacity

        assert!(
            (energy_charged - 1.5).abs() < 0.01,
            "Expected charged energy: 1.5 MWh, Actual charged energy: {}",
            energy_charged
        );
    }

    #[test]
    fn test_load_forecasts_invalid_file() {
        let result = load_forecasts("non_existent_file.json");
        assert!(
            result.is_err(),
            "Expected an error when loading a non-existent file."
        );
    }

    #[test]
    fn test_load_forecasts_invalid_json() {
        // Create a temporary file with invalid JSON
        let temp_file = NamedTempFile::new().unwrap();
        let _ = fs::write(temp_file.path(), "invalid json data");

        let result = load_forecasts(temp_file.path().to_str().unwrap());
        assert!(
            result.is_err(),
            "Expected an error when loading invalid JSON data."
        );
    }

    #[test]
    fn test_load_forecasts_valid_data() {
        // Create a temporary file with valid JSON data
        let temp_file = NamedTempFile::new().unwrap();
        let valid_json = r#"
        {
            "forecasts": [
                {
                    "start": "2022-12-12T00:00:00Z",
                    "end": "2022-12-12T00:15:00Z",
                    "consumption_average_power_interval": 5.0
                }
            ]
        }"#;

        let _ = fs::write(temp_file.path(), valid_json);

        let result = load_forecasts(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(result.forecasts.len(), 1, "Expected to load one forecast.");
        assert_eq!(
            result.forecasts[0].consumption_average_power_interval, 5.0,
            "Expected consumption to match."
        );
    }

    #[test]
    fn test_load_prices_invalid_file() {
        let result = load_day_ahead_prices("non_existent_file.json");
        assert!(
            result.is_err(),
            "Expected an error when loading a non-existent file."
        );
    }

    #[test]
    fn test_load_prices_invalid_json() {
        // Create a temporary file with invalid JSON
        let temp_file = NamedTempFile::new().unwrap();
        let _ = fs::write(temp_file.path(), "invalid json data");

        let result = load_day_ahead_prices(temp_file.path().to_str().unwrap());
        assert!(
            result.is_err(),
            "Expected an error when loading invalid JSON data."
        );
    }

    #[test]
    fn test_load_prices_valid_data() {
        // Create a temporary file with valid JSON data
        let temp_file = NamedTempFile::new().unwrap();
        let valid_json = r#"
        {
            "prices": [
                {
                    "start": "2022-12-12T23:00:00Z",
                    "end": "2022-12-13T00:00:00Z",
                    "market_price_currency": "EUR",
                    "market_price_per_kwh": 0.25
                }
            ]
        }"#;

        let _ = fs::write(temp_file.path(), valid_json);

        // Load prices and average price from the file
        let (prices_data, _average_price) =
            load_day_ahead_prices(temp_file.path().to_str().unwrap()).unwrap();

        // Assert that four 15-minute intervals were generated from one hourly entry
        assert_eq!(
            prices_data.prices.len(),
            4,
            "Expected to load four price entries (15-minute intervals)."
        );

        // Assert that the market price for each interval matches the original hourly price
        for price in prices_data.prices {
            assert_eq!(price.market_price_per_kwh, 0.25, "Expected price to match.");
        }
    }
}
