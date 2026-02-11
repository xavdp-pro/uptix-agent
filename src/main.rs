use sysinfo::{System, Disks};
use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use std::{thread, fs};

#[derive(Serialize, Deserialize, Debug)]
struct SiteConfig {
    url: String,
}

fn get_sites() -> Vec<SiteConfig> {
    let data = fs::read_to_string("sites.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_default()
}

fn send_log(socket: &rust_socketio::client::Client, level: &str, message: &str) {
    let log_data = json!({
        "level": level,
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    let _ = socket.emit("agent_log", log_data);
    println!("[{}] {}", level, message);
}

fn check_site(url: &str) -> String {
    match reqwest::blocking::get(url) {
        Ok(res) => {
            if res.status().is_success() {
                "UP".to_string()
            } else {
                format!("DOWN ({})", res.status())
            }
        }
        Err(e) => format!("DOWN (Error: {})", e),
    }
}

fn main() {
    let host = hostname::get().unwrap_or_else(|_| "unknown".into()).to_string_lossy().to_string();
    let hub_url = "http://localhost:3001";

    println!("Starting Uptix Agent for host: {}", host);

    let socket = ClientBuilder::new(hub_url)
        .on("connect", |_ , _| println!("Connected to Uptix Hub"))
        .on("error", |err, _| eprintln!("Error: {:?}", err))
        .connect()
        .expect("Connection failed");

    send_log(&socket, "INFO", &format!("Agent started on host {}", host));

    let mut sys = System::new_all();

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();
        
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let ram_usage = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
        
        let disks = Disks::new_with_refreshed_list();
        let disk_usage = disks.iter().next().map(|d| {
            let used = d.total_space() - d.available_space();
            (used as f32 / d.total_space() as f32) * 100.0
        }).unwrap_or(0.0);

        let sites_configs = get_sites();
        let mut site_results = Vec::new();

        for config in sites_configs {
            let status = check_site(&config.url);
            if status.contains("DOWN") {
                send_log(&socket, "WARN", &format!("Site {} is DOWN: {}", config.url, status));
            }
            site_results.push(json!({
                "url": config.url,
                "status": status
            }));
        }

        let metrics = json!({
            "server_name": host,
            "cpu_usage": cpu_usage,
            "ram_usage": ram_usage,
            "disk_usage": disk_usage,
            "sites": site_results
        });

        let _ = socket.emit("agent_metrics", metrics);

        thread::sleep(Duration::from_secs(10));
    }
}
