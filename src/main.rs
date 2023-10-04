use std::error::Error;
use std::fs::read_dir;

use dangerous::lock_value;

fn main() -> Result<(), Box<dyn Error>> {
    let zones = read_dir("/sys/devices/virtual/thermal")?;
    for zone in zones {
        let zone = match zone {
            Ok(o) => o,
            Err(_) => continue,
        };

        let mode = zone.path().join("mode");
        if mode.exists() {
            let mode = mode.to_str().unwrap();
            let _ = lock_value(mode, "disabled");
        }
    }

    let _ = lock_value(
        "/sys/class/thermal/thermal_zone0/trip_point_0_temp",
        "6666666",
    );
    let _ = lock_value(
        "/sys/class/thermal/thermal_message/board_sensor_temp",
        "600000",
    );
    let _ = lock_value(
        "/sys/kernel/thermal/max_ttj",
        "MAX_TTJ 6666666 6666666 6666666",
    );
    lock_value("/sys/kernel/thermal/ttj", "TTJ 6666666 6666666 6666666")?;
    let _ = lock_value(
        "/sys/kernel/thermal/min_ttj",
        "MIN_TTJ 6666666 6666666 6666666",
    );
    let _ = lock_value("/sys/kernel/ged/hal/custom_upbound_gpu_freq", "0");

    Ok(())
}
