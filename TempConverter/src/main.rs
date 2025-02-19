const FreezingPoint: f64 = 32.0;

fn fahrenheit_to_celcius(f:f64) -> f64 {
    (f - FreezingPoint) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FreezingPoint
}

fn main() {
    let mut temperatureF = 32.0;
    let temperatureC = fahrenheit_to_celcius(temperatureF);
    println!("{} F is {} C", temperatureF, temperatureC);

    for i in 1..=5{
        temperatureF += 1.0;
        let temperatureC = fahrenheit_to_celcius(temperatureF);
        println!("{} F is {} C", temperatureF, temperatureC)
    }
}
