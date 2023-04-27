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
    },
    Wkg {
        /// Current functional threshold power
        ftp: i32,
        /// Current weight
        weight: f32
    },
    #[structopt(aliases = &["gears", "gi", "gearinches"])]
    GearInches {
        chainring: i32,
        cassette_small: i32, 
        cassette_large: i32,
        as_gearinches: Option<bool>,
    },
    Speed {
        cadence: i32,
        gear_ratio: f32,
        #[structopt(long, default_value = "26.5")]
        wheel_diameter: f32,
    },
    Cadence {
        speed: f32,
        gear_ratio: f32,
        #[structopt(long, default_value = "26.5")]
        wheel_diameter: f32,
    }
}

pub fn main() -> Result<()> {
    let command = Bike::from_args();

    match command {
        Bike::TyrePressure { width, rider_weight, bike_weight } => tyre_pressure(width, rider_weight, bike_weight),
        Bike::PowerZones { ftp } => power_zones(ftp),
        Bike::Wkg { ftp, weight } => watt_per_kilo(ftp, weight),
        Bike::Speed{ cadence, gear_ratio, wheel_diameter } => cadence_to_speed(cadence, gear_ratio, wheel_diameter),
        Bike::Cadence{ speed, gear_ratio, wheel_diameter } => speed_to_cadence(speed, gear_ratio, wheel_diameter),
        Bike::GearInches{ chainring, cassette_small, cassette_large, as_gearinches } => gear_inches(chainring, (cassette_small, cassette_large), as_gearinches),
    }
    Ok(())
}

fn tyre_pressure(tyrewidth: i32, rider_weight: f32, bike_weight: f32) {
    let sysweight = rider_weight + bike_weight;
    let wfront = sysweight * 0.48;
    let wback = sysweight * 0.53;

    let wpow: f64 = (tyrewidth as f64).powf(1.5785);
    let psi_front = ((338.14 * wfront as f64) / wpow) - 7.1685;
    let psi_back = ((338.14 * wback as f64) / wpow) - 7.1685;

    println!("Front: {psi_front:.0} psi\nRear:  {psi_back:.0} psi");
}

fn power_zones(ftp: i32) {
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
    println!("Theoretical W/kg");
    let wkg = ftp as f32 / weight;
    println!("    NOW: {wkg:.1} W/kg");
    for wkg in [4.0, 3.5, 3.0] {
        let power = wkg * weight;
        println!("    for {wkg} W/kg, need {power:.1} W");
    }
    println!();
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

pub fn cadence_to_speed(cadence: i32, gear_ratio: f32, wheel_diameter: f32) {
    let wheel_circumference = wheel_diameter * std::f32::consts::PI;
    let speed = (wheel_circumference * cadence as f32) / (gear_ratio * 60.0);
    println!("{speed:.1} mph");
}

pub fn speed_to_cadence(speed: f32, gear_ratio: f32, wheel_diameter: f32) {
    let wheel_circumference = wheel_diameter * std::f32::consts::PI;
    let cadence = speed * gear_ratio * 60.0 / wheel_circumference;
    println!("{cadence:.1} rpm");
}

fn gear_inches(chainring: i32, cassette: (i32, i32), as_gearinches: Option<bool>) {
    let chainring: &[i32] = &[chainring]; 
    let cassette: &[i32] = &[cassette.0, cassette.1];
    let ratios: Vec<_> = chainring.iter().map(|c| {
        cassette.iter().map(|c2| {
            (*c as f32) / (*c2 as f32)
        }).collect::<Vec<_>>()
    }).collect();
    for (i, first) in ratios.iter().enumerate() {
        let chainring = chainring[i];
        print!("{chainring}: ");
        for (j, second) in first.iter().enumerate() {
            let cassette = cassette[j];
            let second = match as_gearinches{
                Some(true) => *second as f32 * 26.5,
                _ => *second as f32,
            };

            print!("{second:.1} ({cassette}) ");
        }
        println!();
    }
}
