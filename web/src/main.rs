use info_sys;
fn main() {
    let temperature: f32 = info_sys::get_celcius_temperature();
    println!("The temperature is at {temperature}°C");

    let ram = info_sys::get_ram_usage();
    println!(
        "Total memory : {}mo, Free memory : {}mo, Used memory : {}mo",
        ram.total / 1024,
        ram.free / 1024,
        ram.used / 1024
    );

    let storage = info_sys::get_storage_info();
    let free_disk_pourcentage = (storage.free * 100) / storage.total;
    let used_disk_pourcentage = (storage.used * 100) / storage.total;
    const KB_TO_GO: u64 = 1024 * 1024;
    println!(
        "Total disk : {}Go, Free disk : {}Go, Free disk {}%, Used disk {}Go, Used disk {}%",
        storage.total / KB_TO_GO,
        storage.free / KB_TO_GO,
        free_disk_pourcentage,
        storage.used / KB_TO_GO,
        used_disk_pourcentage
    )
}
