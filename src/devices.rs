use cpal::traits::{DeviceTrait, HostTrait};

pub fn list_input_outputs() {
    let host = cpal::default_host();
    let devices = match host.devices() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error getting devices: {}", e);
            return;
        }
    };

    for (i, device) in devices.enumerate() {
        let name = device.name().unwrap_or_else(|_| "Unknown".to_string());

        let supported_configs = device.supported_input_configs();
        let is_input = supported_configs.is_ok_and(|mut sc| sc.next().is_some());

        println!(
            "{}: {} ({})",
            i,
            name,
            if is_input { "Input ✅" } else { "Output 🛑" }
        );
    }
}
