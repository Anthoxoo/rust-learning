use std::fs;

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

fn main() {
    let temperature: f32 = get_celcius_temperature();
    println!("The temperature is at {temperature}°C");

    let ram = get_ram_usage();
    println!(
        "Total mem : {}mo, Free mem : {}mo, Used mem : {}mo",
        ram.total / 1024,
        ram.free / 1024,
        ram.used / 1024
    );
}
