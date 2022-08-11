use regex::Regex;
use std::fs;
use std::process::Command;
use std::str;

pub fn iwlist_scan() {
    let output = Command::new("iwlist")
        .arg("eth0")
        .arg("scanning")
        .output()
        .expect("failed to execute iwlist");

    let output_data = str::from_utf8(&output.stdout)
        .expect("Failed to get stdout")
        .to_owned();
    //println!("iwlist output: {}", output_data);

    //output_data =
    //   fs::read_to_string("/iwlist_data").expect("Something went wrong reading the file");

    let sliced_data = output_data.split("\n");

    let mut main_data: String = String::new();
    let mac_regex =
        Regex::new(r"(\s)|(Cell [0-9][0-9])|(-)|(Address:)").expect("Failed to create mac regex");
    let essid_regex = Regex::new(r"^(.*?)ESSID:").expect("Failed to create essid regex");

    // old regex: (^(.*?)Signal level=)|(\s)
    let signal_regex =
        Regex::new(r"(^(.*?)Signal level[-:=]+)").expect("Failed to create essid regex");

    
    let mut saved_mac: String = String::new();
    let mut saved_essid: String = String::new();
    let mut saved_encryption: String = String::new();
    let mut saved_signal_level: String = String::new();

    for data_chunk in sliced_data {
        if data_chunk.contains("- Address: ") {
            // because order can be random, first check if something is to be printed
            if saved_mac.is_empty() == false
                || saved_essid.is_empty() == false
                || saved_encryption.is_empty() == false
                || saved_signal_level.is_empty() == false
            {
                main_data.push_str(&saved_mac);
                main_data.push_str(&saved_essid);
                main_data.push_str(&saved_encryption);
                main_data.push_str(&saved_signal_level);
                main_data.push_str("%%==SPLIT==%%"); // next wifi network
                main_data.push_str("\n");

                saved_mac = "".to_owned();
                saved_essid = "".to_owned();
                saved_encryption = "".to_owned();
                saved_signal_level = "".to_owned();
            }

            let pure_mac: String = mac_regex.replace_all(data_chunk, "").to_string();
            //println!("pure mac is: \"{}\"", pure_mac);
            saved_mac.push_str(&pure_mac);
            saved_mac.push_str("\n");
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
            saved_essid.push_str(&pure_essid);
            saved_essid.push_str("\n");
            continue;
        }
        if data_chunk.contains("Encryption key:") {
            if data_chunk.contains("key:on") {
                saved_encryption.push_str("true");
            }
            if data_chunk.contains("key:off") {
                saved_encryption.push_str("false");
            }
            saved_encryption.push_str("\n");
            continue;
        }
        if data_chunk.contains("Signal level") == true && data_chunk.contains("Quality") == true {
            // one possibility: Quality:2/5  Signal level:-71 dBm  Noise level:-92 dBm
            // second?: Quality=36/100  Signal level=58/100
            let signal = signal_regex.replace_all(data_chunk, "").to_string();
            let mut pure_signal: String = signal.split_at(3).0.to_string();
            pure_signal = pure_signal.replace("/", "");
            pure_signal = pure_signal.replace(" ", "");
            
            //println!("pure signal: \"{}\"", pure_signal);
            saved_signal_level.push_str(&pure_signal);
            saved_signal_level.push_str("\n");
            continue;
        }
    }
    // Lets also save the last wifi network, because after it no "Address: " exists
    if saved_mac.is_empty() == false
        || saved_essid.is_empty() == false
        || saved_encryption.is_empty() == false
        || saved_signal_level.is_empty() == false
    {
        main_data.push_str(&saved_mac);
        main_data.push_str(&saved_essid);
        main_data.push_str(&saved_encryption);
        main_data.push_str(&saved_signal_level);
        main_data.push_str("%%==SPLIT==%%"); // next wifi network
        main_data.push_str("\n");
    }
    //println!("main data is: \n{}", main_data);

    fs::write("/run/wifi_list_full", output_data).unwrap();
    fs::write("/run/wifi_list_format", main_data).unwrap();
}
