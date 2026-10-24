const LOW_BATTERY: u8 = 20;
const SIGNAL_TIMEOUT_MS: u32 = 3_000;

fn main() {
    let station = "Ground Station";
    let version = (0, 1, 0);
    println!("{station} v{}.{}.{}", version.0, version.1, version.2);

    let drones = [
        (101, 87, 120.5, 200),
        (102, 15, 80.0, 450),
        (103, 64, 0.0, 5_200),
    ];

    let mut alerts = 0;
    let mut total_altitude = 0.0;

    for (id, battery, altitude, signal_age_ms) in drones {
        let status = drone_status(battery, signal_age_ms);
        println!("Дрон #{id}: заряд {battery}%, висота {altitude:.1} м — {status}");

        if status != "OK" {
            alerts += 1;
        }
        total_altitude += altitude;
    }

    let avg = average(total_altitude, drones.len());
    println!("Тривог: {alerts}, середня висота: {avg:.1} м");
}

fn drone_status(battery: u8, signal_age_ms: u32) -> &'static str {
    if signal_age_ms > SIGNAL_TIMEOUT_MS {
        "ЗВ'ЯЗОК ВТРАЧЕНО"
    } else if battery < LOW_BATTERY {
        "НИЗЬКИЙ ЗАРЯД"
    } else {
        "OK"
    }
}

fn average(sum: f32, count: usize) -> f32 {
    if count == 0 {
        return 0.0;
    }
    sum / count as f32
}