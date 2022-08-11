pub mod iwlist_wrapper;
pub mod wlarm_le_wrapper;

use crate::iwlist_wrapper::iwlist_scan;
use crate::wlarm_le_wrapper::wlarm_le_scan;

use std::fs;
use std::process::Command;
use std::{thread, time};

/*
/run/wifi_list_full - unformatted output of the main command, so the user can view it
/run/wifi_list_format - formatted, same output for all devicess

syntax of advanced wifi scan
any value can be none, it will be handled by the gui

mac
essid - the only thing that is really needed
a bool if a password is needed. none is also handled
wifi strength ( RSSI ) - higher better
"%%==SPLIT==%%" - string, to simply split() it in other programs. dont you ever try an ESSID with that exact name.

*/

fn main() {
    // Testing
    //wlarm_le_scan();
    //iwlist_scan();
    //exit(0);

    let model =
        fs::read_to_string("/opt/inkbox_device").expect("Something went wrong reading the file");

    //println!("model is: {}", model);

    let mut WIFI_MODULE: String = String::new();
    let mut SDIO_WIFI_PWR_MODULE: String = String::new();
    let mut WIFI_DEV: String = String::new();

    if model == "n873" || model == "n236" || model == "n306" {
        WIFI_MODULE = "/modules/wifi/8189fs.ko".to_owned();
        SDIO_WIFI_PWR_MODULE = "/modules/drivers/mmc/card/sdio_wifi_pwr.ko".to_owned();
        WIFI_DEV = "eth0".to_owned();
    } else if model == "n705" || model == "n905b" || model == "n905c" || model == "n613" {
        WIFI_MODULE = "/modules/dhd.ko".to_owned();
        SDIO_WIFI_PWR_MODULE = "/modules/sdio_wifi_pwr.ko".to_owned();
        WIFI_DEV = "eth0".to_owned();
    } else if model == "n437" {
        WIFI_MODULE = "/modules/wifi/bcmdhd.ko".to_owned();
        SDIO_WIFI_PWR_MODULE = "/modules/drivers/mmc/card/sdio_wifi_pwr.ko".to_owned();
        WIFI_DEV = "wlan0".to_owned();
    } else if model == "kt" {
        WIFI_MODULE = "ar6003".to_owned();
        WIFI_DEV = "wlan0".to_owned();
    } else {
        WIFI_MODULE = "/modules/dhd.ko".to_owned();
        SDIO_WIFI_PWR_MODULE = "/modules/sdio_wifi_pwr.ko".to_owned();
        WIFI_DEV = "eth0".to_owned();
    }
    //println!("WIFI_MODULE is: {}", WIFI_MODULE);
    //println!("SDIO_WIFI_PWR_MODULE is: {}", SDIO_WIFI_PWR_MODULE);
    //println!("WIFI_DEV is: {}", WIFI_DEV);

    if model != "kt" {
        insmod(&SDIO_WIFI_PWR_MODULE);
        insmod(&WIFI_DEV);
    } else {
        Command::new("modprobe")
            .arg("-r")
            .arg(&WIFI_MODULE)
            .output()
            .expect("failed to execute iwlist");
    }
    // Race condition
    thread::sleep(time::Duration::from_millis(1500));
    if model == "n705" || model == "n905b" || model == "n905c" || model == "n613" || model == "n437"
    {
        Command::new("wlarm_le")
            .arg("up")
            .output()
            .expect("failed to execute iwlist");
        wlarm_le_scan();
    } else {
        Command::new("ifconfig")
            .arg(WIFI_DEV)
            .arg("up")
            .output()
            .expect("failed to execute iwlist");
        iwlist_scan();
    }
}

pub fn insmod(module: &str) {
    let new_command = Command::new("insmod")
        .arg(module)
        .output()
        .expect("failed to execute iwlist");
    //println!("insmod exit status is: {:?}", new_command.status);
}
