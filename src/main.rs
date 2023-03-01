use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Suggested tyre pressure (assuming 45:55 load)
enum Bike {
    #[structopt(aliases = &["tyres", "tyre", "pressure"])]
    TyrePressure{
        /// Tyre width, in mm
        width: i32,
        /// Rider weight, in kg
        rider_weight: f32,
        /// Bike weight, in kg
        bike_weight: f32,
    },
    #[structopt(aliases = &["power", "zones"])]
    PowerZones {
        /// Current functional threshold power
        ftp: i32,
        /// Current weight
        weight: f32,
    },
    Wkg {
        /// Current functional threshold power
        ftp: i32,
        /// Current weight
        weight: f32
    },
    GearInches {

    }
}

pub fn main() -> Result<()> {
    let command = Bike::from_args();

    match command {
        Bike::TyrePressure { width, rider_weight, bike_weight } => tyre_pressure(width, rider_weight, bike_weight),
        Bike::PowerZones { ftp, weight} => power_zones(ftp, weight),
        Bike::Wkg { ftp, weight } => watt_per_kilo(ftp, weight),
        _ => todo!(),
    }
    Ok(())
}

fn tyre_pressure(tyrewidth: i32, rider_weight: f32, bike_weight: f32) {
    let sysweight = rider_weight + bike_weight;
    let wfront = sysweight * 0.45;
    let wback = sysweight * 0.55;

    let wpow: f64 = (tyrewidth as f64).powf(1.5785);
    let psi_front = ((338.14 * wfront as f64) / wpow) - 7.1685;
    let psi_back = ((338.14 * wback as f64) / wpow) - 7.1685;

    println!("Front: {psi_front:.0} psi\nRear:  {psi_back:.0} psi");
}

fn power_zones(ftp: i32, weight: f32) {
    let zones = [
        ("Recovery", 0, 55),
        ("Endurance", 55, 75),
        ("Sweetspot", 75, 90),
        ("Threshold", 90, 105),
        ("VO2 Max", 105, 120),
        ("Anaerobic", 120, 150),
    ];
    let norm_ftp = ftp as f32 / 100.0;
    println!("Power zones at {ftp:.0}W ftp");
    for (name, start, end) in zones {
        let (start, end) = (start as f32 * norm_ftp, end as f32 * norm_ftp);
        println!("    {name:15} {start:3.0} to {end:3.0} W");
    }
    let name = "Neuromuscular";
    let start = 150.0 * norm_ftp;
    println!("    {name:15} {start:3.0}+ W");
    println!();
    
}

fn watt_per_kilo(ftp: i32, weight: f32) {
    ftp_for_wkg(4.0, weight);
    ftp_for_wkg(3.5, weight);
}

fn ftp_for_wkg(wkg: f32, weight: f32) {
    println!("For {wkg} W/kg...");
    for sub in 1..=5 {
        let delta = weight - (sub as f32);
        let power = delta * wkg;
        println!("...at {delta} kg, need {power:.0}W");
    }
    println!();
}
