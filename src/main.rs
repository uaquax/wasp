#[macro_use]
extern crate rocket;

use rocket::http::Method;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_cors::{AllowedOrigins, CorsOptions};
use serde_json::{json, Value};
use sysinfo::{CpuExt, DiskExt, NetworkExt, System, SystemExt};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Network {
    pub name: String,
    pub total_income: u64,
    pub total_outcome: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Disk {
    pub name: String,
    pub size: u64,
    pub free: u64,
    pub file_system: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SysInfo {
    pub cpu: Value,
    pub ram: u64,
    pub ram_used: u64,
    pub uptime: u64,
    pub os: Option<String>,
    pub networks: Vec<Network>,
    pub disks: Vec<Disk>,
}

#[get("/api/sysinfo")]
fn index() -> Json<SysInfo> {
    let sys = System::new_all();
    let cpu = sys.cpus().first().unwrap();
    let networks = sys
        .networks()
        .into_iter()
        .map(|net| Network {
            name: net.0.to_string(),
            total_income: net.1.total_received(),
            total_outcome: net.1.total_transmitted(),
        })
        .collect();
    let disks = sys
        .disks()
        .into_iter()
        .map(|disk| Disk {
            name: disk.name().to_string_lossy().to_string(),
            size: disk.total_space(),
            free: disk.available_space(),
            file_system: std::str::from_utf8(disk.file_system()).unwrap().to_string(),
        })
        .collect();

    Json(SysInfo {
        cpu: json!({"brand": cpu.brand(), "frequency":  cpu.frequency()}),
        ram: sys.total_memory(),
        ram_used: sys.used_memory(),
        uptime: sys.uptime(),
        os: sys.long_os_version(),
        networks: networks,
        disks: disks,
    })
}

#[launch]
fn rocket() -> _ {
    // let cors = CorsOptions::default().allow_credentials(true);
    rocket::build()
        // .manage(cors.to_cors())
        .mount("/", routes![index])
}
