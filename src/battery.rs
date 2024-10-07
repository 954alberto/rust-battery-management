use anyhow::{anyhow, Result};
use log::{info, warn};

/// A struct representing a battery with specific properties.
pub struct Battery {
    capacity: f64,   // Max capacity in MWh
    pub charge: f64, // Current charge in MWh
    max_rate: f64,   // Max charging/discharging rate in MW
    efficiency: f64, // Efficiency in charging/discharging
}

impl Battery {
    /// Creates a new `Battery` instance with predefined properties.
    ///
    /// # Returns
    /// A `Battery` instance initialized with a capacity of 3.0 MWh,
    /// a starting charge of 1.5 MWh, a max charging/discharging rate of 1.5 MW,
    /// and an efficiency of 90%.
    // pub fn new() -> Self {
    //     Battery {
    //         capacity: 3.0, // Ensure this is set to 3 MWh
    //         charge: 1.5,   // Initial charge can be adjusted as needed
    //         max_rate: 1.5,
    //         efficiency: 0.90,
    //     }
    // }
    pub fn new(capacity: f64, initial_charge: f64, max_rate: f64, efficiency: f64) -> Self {
        Battery {
            capacity,
            charge: initial_charge,
            max_rate,
            efficiency,
        }
    }

    /// Charges the battery with the specified amount of power for a given duration.
    ///
    /// # Parameters
    /// - `amount_mw`: The amount of power in megawatts (MW) to charge the battery.
    /// - `duration_hours`: The duration for which to charge the battery, in hours.
    ///
    /// # Returns
    /// The amount of energy charged in megawatt-hours (MWh), wrapped in a `Result`.
    /// If the amount of power is negative, it returns an error.
    ///
    /// # Errors
    /// Returns an error if `amount_mw` is negative.
    pub fn charge_battery(&mut self, amount_mw: f64, duration_hours: f64) -> Result<f64> {
        if amount_mw < 0.0 {
            warn!("Attempted to charge with a negative power: {}", amount_mw);
            return Err(anyhow!(
                "Attempted to charge with a negative power: {}",
                amount_mw
            ));
        }

        // Ensure charging rate does not exceed max_rate
        let effective_mw = amount_mw.min(self.max_rate); // Limit to max_rate
        let energy_to_battery = effective_mw * duration_hours; // Total energy input
        let actual_energy = energy_to_battery * self.efficiency; // Effective energy due to efficiency

        info!(
            "Charging with: {} MW for {} hours. Total energy to battery: {}, Effective energy (after efficiency): {}",
            effective_mw, duration_hours, energy_to_battery, actual_energy
        );

        // Calculate how much energy can be stored based on capacity
        let available_capacity = self.capacity - self.charge; // Remaining capacity
        let energy_stored = actual_energy.min(available_capacity); // Store only what can fit

        info!(
            "Available capacity: {}, Energy stored: {}",
            available_capacity, energy_stored
        );

        self.charge += energy_stored; // Add usable energy to the charge

        // Ensure we do not exceed capacity
        if self.charge > self.capacity {
            self.charge = self.capacity;
        }

        info!("New charge after charging: {} MW", self.charge);

        Ok(energy_stored) // Return the actual energy added
    }

    /// Discharges the battery by the specified amount of power for a given duration.
    ///
    /// # Parameters
    /// - `amount_mw`: The amount of power in megawatts (MW) to discharge from the battery.
    /// - `duration_hours`: The duration for which to discharge the battery, in hours.
    ///
    /// # Returns
    /// The amount of energy discharged in megawatt-hours (MWh), wrapped in a `Result`.
    /// If the amount of power is negative, it returns an error.
    ///
    /// # Errors
    /// Returns an error if `amount_mw` is negative.
    pub fn discharge_battery(&mut self, amount_mw: f64, duration_hours: f64) -> Result<f64> {
        if amount_mw < 0.0 {
            warn!(
                "Attempted to discharge with a negative power: {}",
                amount_mw
            );
            return Err(anyhow!(
                "Attempted to discharge with a negative power: {}",
                amount_mw
            ));
        }

        // Ensure discharging rate does not exceed max_rate
        let effective_mw = amount_mw.min(self.max_rate); // Limit to max_rate
        let energy_needed = effective_mw * duration_hours; // Total energy needed
        let actual_energy_needed = energy_needed / self.efficiency; // Adjust for efficiency

        if self.charge < actual_energy_needed {
            let discharged = self.charge; // Discharge only what's available
            self.charge = 0.0; // Set charge to zero
            info!("Discharged all available energy: {} MWh", discharged);
            Ok(discharged) // Return how much was discharged
        } else {
            self.charge -= actual_energy_needed; // Reduce charge based on energy needed
            info!(
                "Discharged energy: {} MWh, Remaining charge: {} MWh",
                actual_energy_needed, self.charge
            );
            Ok(actual_energy_needed) // Return the actual energy discharged
        }
    }
}
