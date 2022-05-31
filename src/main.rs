use libnotify;
use libnotify_sys;
use std::{thread, time, env};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Eq, PartialEq)]
enum BatteryStatus {
    CHARGING,
    DISCHARGING,
    OTHER
}

struct BatteryInfos {
    name: String,
    status: BatteryStatus,
    capacity: u8
}

const MILLI_TO_SECOND: i32 = 1000;
const URGENCY: libnotify::Urgency = libnotify::Urgency::Critical;
const UEVENT_BATTERY_FILE: &str = "/sys/class/power_supply/BAT1/uevent";
const POWER_SUPPLY_NAME: &str = "POWER_SUPPLY_NAME";
const POWER_SUPPLY_CAPACITY: &str = "POWER_SUPPLY_CAPACITY";
const POWER_SUPPLY_STATUS: &str = "POWER_SUPPLY_STATUS";
const CHARGING: &str = "Charging";
const DISCHARGING: &str = "Discharging";

fn send_notification(msg: String, timeout: i32) -> libnotify::Notification {
    let notif = libnotify::Notification::new(&msg, None, None);

    notif.set_timeout(timeout * MILLI_TO_SECOND);
    notif.set_urgency(URGENCY);

    notif.show().unwrap();
    return notif;
}

fn update_notification(notif: &libnotify::Notification, msg: String, timeout: i32) {
    let time_to_wait = time::Duration::from_secs(timeout as u64);
    thread::sleep(time_to_wait);

    notif.update(&msg, None, None).unwrap();
    notif.show().unwrap();
}

fn get_battery_infos(uevent_file: String) -> BatteryInfos{
    let file = File::open(uevent_file)
        .expect("Impossible de lire le fichier.");
    let buf_reader = BufReader::new(file);

    let mut bat_infos = BatteryInfos {
        name: String::from("UNDEFINED"),
        status: BatteryStatus::OTHER,
        capacity: 0
    };

    for line in buf_reader.lines() {
        let l = line.unwrap();
        let octets = l.as_bytes();

        for (i, &e) in octets.iter().enumerate() {
            if e == b'=' {
                let name = &l[0..i];
                let result = &l[i+1..];

                if name.eq(POWER_SUPPLY_CAPACITY) {
                    bat_infos.capacity = result.parse::<u8>().unwrap();
                } else if name.eq(POWER_SUPPLY_STATUS) {
                    if result.eq(CHARGING) {
                        bat_infos.status = BatteryStatus::CHARGING;
                    } else if result.eq(DISCHARGING) {
                        bat_infos.status = BatteryStatus::DISCHARGING;
                    } else {
                        bat_infos.status = BatteryStatus::OTHER;
                    }
                } else if name.eq(POWER_SUPPLY_NAME) {
                    bat_infos.name = result.to_string();
                }
            }
        }
    }

    return bat_infos;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Wrong arguments. Usage: {} <limit_percentage>", args[0]);
    }

    let limit_percentage = args[1].parse::<u8>().unwrap();

    libnotify::init("Batterie faible").unwrap();

    loop {
        let mut bat_infos = get_battery_infos(UEVENT_BATTERY_FILE.to_string());

        if bat_infos.status == BatteryStatus::DISCHARGING
        && bat_infos.capacity <= limit_percentage {

            let notif = send_notification(format!("Restant: {}%.", bat_infos.capacity), libnotify_sys::NOTIFY_EXPIRES_NEVER);

            loop {
                let time_to_wait = time::Duration::from_secs(5 as u64);
                thread::sleep(time_to_wait);

                bat_infos = get_battery_infos(UEVENT_BATTERY_FILE.to_string());
                if bat_infos.status != BatteryStatus::DISCHARGING || bat_infos.capacity > limit_percentage {
                    notif.close().unwrap();
                    break;
                }

                update_notification(&notif, format!("Restant: {}%.", bat_infos.capacity), 0);
            }

        }

        let time_to_wait = time::Duration::from_secs(5 as u64);
        thread::sleep(time_to_wait);
    }
}
