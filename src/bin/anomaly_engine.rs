use tubes_pf_modul_2::anomaly;

fn main() {
    let temperature = 85.0;
    let pressure = 110.0;
    let vibration = 60.0;

    let current_time = 30;
    let last_time = 20;
    let interval = 10;

    println!("================================");
    println!("     RULE & ANOMALY ENGINE");
    println!("================================");

    println!(
        "Temperature : {} °C → {}",
        temperature,
        anomaly::check_temperature(temperature)
    );

    println!(
        "Pressure    : {} → {}",
        pressure,
        anomaly::check_pressure(pressure)
    );

    println!(
        "Vibration   : {} → {}",
        vibration,
        anomaly::check_vibration(vibration)
    );

    println!("--------------------------------");

    if anomaly::check_interval(
        current_time,
        last_time,
        interval,
    ) {
        println!("Sampling    : INTERVAL REACHED");
        println!("Sensor data can be checked.");
    } else {
        println!("Sampling    : NOT YET");
    }

    let data = anomaly::SensorData {
        temperature,
        pressure,
        vibration,
    };

    let results = anomaly::check_anomaly(&data);

    println!("--------------------------------");

    if results.is_empty() {
        println!("Status      : ALL SENSORS NORMAL");
    } else {
        println!("Status      : ANOMALY DETECTED");

        for result in results {
            println!("- {}", result);
        }
    }

    println!("================================");
}