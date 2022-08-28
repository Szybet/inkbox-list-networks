pub mod iwlist_wrapper;
pub mod wlarm_le_wrapper;

use crate::iwlist_wrapper::iwlist_scan;
use crate::wlarm_le_wrapper::wlarm_le_scan;

use std::fs;

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
        fs::read_to_string("/opt/inkbox_device").expect("Something went wrong reading the file").replace('\n', "");

    //println!("model is: {}", model);
    /* 
    Command::new("/usr/local/bin/wifi/toggle.sh")
    .arg("on")
    .output()
    .expect("failed to execute iwlist");
    */
    
    if model == "n437"
    {
        wlarm_le_scan();
    } else {
        iwlist_scan();
    }
}
