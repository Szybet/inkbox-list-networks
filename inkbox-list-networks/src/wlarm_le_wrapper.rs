use std::process::Command;
use regex::Regex;
use std::fs;
use std::str;

pub fn wlarm_le_scan() {
    //println!("wlarm_le wrapper");

    // Because Glo HD is finicky in every possible way
    Command::new("ifconfig")
        .arg("wlan0")
        .arg("up")
        .output()
        .expect("failed to execute ifconfig");

    let output = Command::new("wlarm_le")
        .arg("escanresults")
        .output()
        .expect("failed to execute wlarm_le");

    let output_data = str::from_utf8(&output.stdout).expect("Failed to get stdout").to_owned();

    //let output_data =
    //    fs::read_to_string("/wlarm_le_data").expect("Something went wrong reading the file");

    ////println!("iwlist output: {}", output_data);

    let sliced_data = output_data.split("\n");
    let mut main_data: String = String::new();
    let rssi_regex = Regex::new(r"(\D+\d?\D)").expect("Failed to create essid regex");
    let mac_regex = Regex::new(r"(?:[A-Fa-f0-9]{2}[:-]){5}(?:[A-Fa-f0-9]{2})")
        .expect("Failed to create essid regex");

    let mut encryption_is_next = false;

    let mut saved_ssid: String = String::new();
    let mut saved_mac: String = String::new();
    let mut saved_rssi: String = String::new();
    
    for data_chunk in sliced_data {
        if data_chunk.contains("SSID: ") && !data_chunk.contains("BSSID: ") {
            let mut pure_ssid: String = data_chunk.to_string();
            pure_ssid.pop();
            for _ in 0..7 {
                pure_ssid.remove(0);
            }
            if pure_ssid.is_empty() {
                continue;
            }
            //println!("pure_ssid is: \"{}\"", pure_ssid);
            saved_ssid = "".to_owned();
            saved_ssid.push_str(&pure_ssid);
            saved_ssid.push('\n');
            continue;
        }
        if data_chunk.contains("RSSI") {
            let rssi: String = rssi_regex.replace_all(data_chunk, "").to_string();
            let splitted_rssi = rssi.split_at(2);
            let pure_rssi = splitted_rssi.0.to_owned();
            saved_rssi = "".to_owned();
            if rssi.is_empty() || rssi.chars().count() < 4 {
                //println!("rssi is weird or to low: {}", rssi);
                saved_rssi.push('0');
            } else {
                //println!("pure_rssi is: {}", pure_rssi);
                saved_rssi.push_str(&pure_rssi);
            }
            saved_rssi.push('\n');
            continue;
        }
        if data_chunk.contains("BSSID: ") {
            let mut pure_mac: String = String::new();
            for cap in mac_regex.captures_iter(data_chunk) {
                pure_mac = cap[0].to_owned();
            }
            //println!("pure mac is: {}", pure_mac);
            saved_mac = "".to_owned();
            saved_mac.push_str(&pure_mac);
            saved_mac.push('\n');
            continue;
        }
        if data_chunk.contains("Supported Rates:") {
            encryption_is_next = true;
            continue;
        }
        if encryption_is_next {
            encryption_is_next = false;
            
            let encryption: String = if data_chunk.contains("RSN:") || data_chunk.contains("WPA:") {
                "true".to_owned()
            } else {
                "false".to_owned()
            };
            
            main_data.push_str(&saved_mac);
            main_data.push_str(&saved_ssid);
            main_data.push_str(&encryption);
            main_data.push('\n');
            main_data.push_str(&saved_rssi);
            main_data.push_str("%%==SPLIT==%%"); // next wifi network
            main_data.push('\n');
        }
    }
    //println!("main_data is: \n{}", main_data);
    fs::write("/run/wifi_list_full", output_data).unwrap();
    fs::write("/run/wifi_list_format", main_data).unwrap();
}
