extern crate uom;

use uom::si::f64::*;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::pressure::hectopascal;
use uom::fmt::DisplayStyle::Abbreviation;

fn main() {

    let temperature = ThermodynamicTemperature::new::<degree_celsius>(20.0_f64);
    let celsius = ThermodynamicTemperature::format_args(degree_celsius, Abbreviation);
    let h_pa = Pressure::format_args(hectopascal, Abbreviation);

    println!(
        "SVP buck: {}",
        h_pa.with(saturation_water_vapor_pressure(temperature))
    );

    println!("VP : {}", h_pa.with(vapor_pressure(35.4_f64, temperature)));

    println!("Dew point buck: {}", celsius.with(dew_point(35.4_f64, temperature)));
    println!("wet bulb temperature: {}", celsius.with(wet_bulb_temperature(35.4_f64, temperature)))

}


fn saturation_water_vapor_pressure(temperature: ThermodynamicTemperature) -> Pressure {
    Pressure::new::<hectopascal>(
    6.1121_f64
        * ((18.678_f64 - (temperature.get::<degree_celsius>() / 234.5_f64))
            * (temperature.get::<degree_celsius>() / (257.14_f64 + temperature.get::<degree_celsius>())))
            .exp())
}

fn vapor_pressure(relative_air_humidity: f64, temperature: ThermodynamicTemperature) -> Pressure {
    Pressure::new::<hectopascal>(
    100.0_f64 / relative_air_humidity
        * saturation_water_vapor_pressure(temperature).get::<hectopascal>())
}


fn dew_point(relative_air_humidity_percent: f64, temperature: ThermodynamicTemperature) -> ThermodynamicTemperature {
    const B: f64 = 18.678;
    const C: f64 = 257.14;
    const D: f64 = 234.5;

    let gamma: f64 = (relative_air_humidity_percent / 100.0_f64
        * ((B - temperature.get::<degree_celsius>() / D) * (temperature.get::<degree_celsius>() / (C + temperature.get::<degree_celsius>())))
            .exp())
    .ln();

    ThermodynamicTemperature::new::<degree_celsius>( C * gamma / (B - gamma))
}

fn wet_bulb_temperature(relative_air_humidity_percent: f64, temperature: ThermodynamicTemperature) -> ThermodynamicTemperature {
    ThermodynamicTemperature::new::<degree_celsius>(
        temperature.get::<degree_celsius>() * (0.151977_f64 * (relative_air_humidity_percent + 8.313659_f64).powf(0.5)).atan()
        + (temperature.get::<degree_celsius>()+ relative_air_humidity_percent).atan()
        - (relative_air_humidity_percent - 1.676331_f64).atan()
        + 0.00391838 * relative_air_humidity_percent.powf(1.5) * (0.023101_f64 * relative_air_humidity_percent).atan()
        - 4.686035_f64
    
    )
}
