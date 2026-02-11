use info_sys::{disk, memory, os, temperature};

fn display_info() {
    let temperature: f32 = temperature::get_celcius_temperature();
    println!("The temperature is at {temperature}°C");

    let ram = memory::get_ram_usage();
    println!(
        "Total memory : {}mo, Free memory : {}mo, Used memory : {}mo",
        ram.total, ram.free, ram.used,
    );

    let storage = disk::get_storage_info();
    println!(
        "Total disk : {}Go, Free disk : {}Go, Free disk {}%, Used disk {}Go, Used disk {}%",
        storage.total, storage.free, storage.free_percentage, storage.used, storage.used_percentage,
    );
    println!("Linux distro : {}", os::name());
    println!("Total computer uptime : {}", os::uptime());
}

fn main() {
    display_info();
}
