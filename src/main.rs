use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Suggested tyre pressure (assuming 45:55 load)
enum Bike {
    TyrePressure{
    /// Tyre width, in mm
    width: i32,
    /// Rider weight, in kg
    rider_weight: f32,
    /// Bike weight, in kg
    bike_weight: f32,
    }
}

pub fn main() -> Result<()> {
    let command = Bike::from_args();

    match command {
        Bike::TyrePressure { width, rider_weight, bike_weight } => {
            let sysweight = rider_weight + bike_weight;
            let wfront = sysweight * 0.45;
            let wback = sysweight * 0.55;

            let wpow: f64 = (width as f64).powf(1.5785);
            let psi_front = ((338.14 * wfront as f64) / wpow) - 7.1685;
            let psi_back = ((338.14 * wback as f64) / wpow) - 7.1685;

            println!("Front: {psi_front:.0} psi\nRear:  {psi_back:.0} psi");
        }
    }
        Ok(())
}
