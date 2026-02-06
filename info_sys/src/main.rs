use std::fs;
use std::process::Command;

fn get_celcius_temperature() -> f32 {
    let path: String = String::from("/sys/class/thermal/thermal_zone0/temp");
    let raw_temperature = fs::read_to_string(path).expect("Error while reading the file");

    let final_temperature: f32 = raw_temperature
        .trim()
        .parse()
        .expect("error while converting to float");
    return final_temperature / 1000.0;
}
struct RamInfo {
    total: u32,
    free: u32,
    used: u32,
}
fn get_ram_usage() -> RamInfo {
    let path: String = String::from("/proc/meminfo");
    let reader = fs::read_to_string(path).expect("Error while reading the file");

    let line_total_mem: &str = reader
        .lines()
        .nth(0)
        .expect("This index of line does not exists.");

    let line_free_mem: &str = reader
        .lines()
        .nth(2)
        .expect("This index of line does not exists.");

    let total_mem: u32 = line_total_mem
        .split_whitespace()
        .nth(1)
        .expect("No value found.")
        .parse()
        .expect("Not a number.");

    let free_mem: u32 = line_free_mem
        .split_whitespace()
        .nth(1)
        .expect("No value found.")
        .parse()
        .expect("Not a number.");

    let used_mem: u32 = total_mem - free_mem;

    return RamInfo {
        total: total_mem,
        free: free_mem,
        used: used_mem,
    };
}
struct DiskInfo {
    total: u64,
    free: u64,
    used: u64,
}
fn get_storage_info() -> DiskInfo {
    let storage_command_output = Command::new("df")
        .arg("-k")
        .arg(".") // shows the main disk
        .output()
        .expect("Error reading df -k command.");

    let stdout_to_string: String = String::from_utf8(storage_command_output.stdout)
        .expect("Error converting to utf-8 df -k command.");

    let disk_line: &str = stdout_to_string
        .lines()
        .nth(1)
        .expect("Error reading line 1 of df -k");

    let total_storage: u64 = disk_line
        .split_whitespace()
        .nth(1)
        .expect("Error reading total storage")
        .parse()
        .expect("Error converting total storage to u64");

    let used_storage: u64 = disk_line
        .split_whitespace()
        .nth(2)
        .expect("Error reading used storage")
        .parse()
        .expect("Error converting used storage to u64");

    let available_storage: u64 = disk_line
        .split_whitespace()
        .nth(3)
        .expect("Error reading available storage")
        .parse()
        .expect("Error converting available storage to u64");

    return DiskInfo {
        total: total_storage,
        free: available_storage,
        used: used_storage,
    };
}

fn main() {
    let temperature: f32 = get_celcius_temperature();
    println!("The temperature is at {temperature}°C");

    let ram = get_ram_usage();
    println!(
        "Total memory : {}mo, Free memory : {}mo, Used memory : {}mo",
        ram.total / 1024,
        ram.free / 1024,
        ram.used / 1024
    );

    let storage = get_storage_info();
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
