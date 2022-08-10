use std::process::Command;
use std::str;
use regex::Regex;
use std::fs;

pub fn iwlist_scan() {
    let output = Command::new("iwlist")
    .arg("eth0")
    .arg("scanning")
    .output()
    .expect("failed to execute iwlist");

    let output_data = str::from_utf8(&output.stdout).expect("Failed to get stdout").to_owned();
    //println!("iwlist output: {}", output_data);
    let sliced_data = output_data.split("\n");
    
    let mut main_data: String = String::new();
    let mac_regex = Regex::new(r"(\s)|(Cell [0-9][0-9])|(-)|(Address:)").expect("Failed to create mac regex");
    let essid_regex = Regex::new(r"^(.*?)ESSID:").expect("Failed to create essid regex");
    let signal_regex = Regex::new(r"(^(.*?)Signal level=)|(\s)").expect("Failed to create essid regex");

    for data_chunk in sliced_data {
        if data_chunk.contains("- Address: ") {
            let pure_mac: String = mac_regex.replace_all(data_chunk, "").to_string();
            //println!("pure mac is: \"{}\"", pure_mac);
            main_data.push_str(&pure_mac);
            main_data.push_str("\n");
            continue;
        }
        if data_chunk.contains("ESSID:") {
            // To avoid confussion when a essid is named ESSID:, its removing first 7 characters and the last one
            let mut pure_essid: String = data_chunk.to_string();
            pure_essid = essid_regex.replace_all(&pure_essid, "").to_string();
            
            // There is a function for that with a funny name!
            pure_essid.pop(); // remove last character
            pure_essid.remove(0);

            //println!("pure essid is: \"{}\" ", pure_essid);
            main_data.push_str(&pure_essid);
            main_data.push_str("\n");
            continue;
        }
        if data_chunk.contains("Encryption key:") {
            if data_chunk.contains("key:on") {
                main_data.push_str("true");
            }
            if data_chunk.contains("key:off") {
                main_data.push_str("false");
            }
            main_data.push_str("\n");
            continue;
        }
        if data_chunk.contains("Signal level=") {
            let mut pure_signal = signal_regex.replace_all(data_chunk, "").to_string();
            pure_signal = pure_signal.replace(" ", "");
            pure_signal = pure_signal.replace("/100", "");
            //println!("pure signal: \"{}\"", pure_signal);
            main_data.push_str(&pure_signal);
            main_data.push_str("\n");
            main_data.push_str("%%==SPLIT==%%"); // next wifi network
            main_data.push_str("\n");
            continue;
        }
    }
    //println!("main data is: \n{}", main_data);

    fs::write("/run/wifi_list_full", output_data).unwrap();
    fs::write("/run/wifi_list_format", main_data).unwrap();

}
