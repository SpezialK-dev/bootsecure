use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use notify_rust::{Notification, Hint, NotificationHandle};
use std::path::PathBuf;

fn send_notification(notification_string :&str) ->  Result<NotificationHandle, notify_rust::error::Error>{
    print!("{notification_string}");
    Notification::new()
        .summary("bootsecure bootwarning")
        .body(notification_string)
        .appname("bootsecure")
        .hint(Hint::Resident(true))
        .timeout(0)
        .show()
}

// all paarsing for monotonic counter
fn correct_uefi_var(file_name : &str) -> bool{
    let mtc_vars = ["MTC", "MonotonicCounter"];
    for poss_var in mtc_vars{
        if file_name.contains(poss_var){
            return true;
        }
    }
    false

}

#[cfg(target_os="linux")]
fn read_mtc(value :&mut u16) -> std::io::Result<()>{

    // this has to iterate through the dir to find if either MTC (lenovo) or MonotonicCounter (Asus) is encountered.
    if let Ok(entries) = fs::read_dir("/sys/firmware/efi/efivars/") {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                if let Ok(filename) = filename.into_string(){
                    if correct_uefi_var(&filename){
                        let mut uefi_var = File::open(entry.path())?;// no need to optimize cause the file should never be larger than a few bytes
                        let mut mtc_data = Vec::new();
                        uefi_var.read_to_end(&mut mtc_data)?;
                        *value =  u16::from_le_bytes(mtc_data[4..6].try_into().unwrap());
                        return Ok(())
                    }
                }
            }
        }
    }
    *value = 0;
    Ok(())
}
//storing and and restoring the value to a config file
//we want the value to be consumed when writing since its the last action being done
fn write_value_to_config(data: String, path_mtc: &PathBuf )-> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path_mtc)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn read_value_from_mtc_val(value :&mut u16, path_mtc: &PathBuf)->std::io::Result<()>{
    let exists = fs::exists(path_mtc).unwrap_or_default();
    if exists{
        let content_mtc = match fs::read_to_string(path_mtc){
            Ok(val) => val,
            Err(err) => {print!("The following error occued while reading {err}");String::from("0")} //if 0 is returned from config file
        };
        *value = match content_mtc.parse::<u16>(){
        Ok(val) => val,
            // the Err Val gets triggered when file gets opend with text editor that adds null byte at end (ex. vim) this is a feature not a bug
            Err(_) => {if let Err(err) = send_notification("Tempering with Savefile has occured!"){ print!("{err}")}; 0}
        }
    }else {
        *value = 0;
    }
    Ok(())
}


fn main() -> std::io::Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bootsecure");
    let mtc_value = xdg_dirs.place_data_file("mtc_value")
        .expect("cannot create bootsecure directory");

    let mut old_mtc : u16 = 0;
    let mut current_mtc : u16 = 0;
    let _ = read_value_from_mtc_val(&mut old_mtc, &mtc_value);
    let _ = read_mtc(&mut current_mtc);
    let _ =write_value_to_config(current_mtc.to_string(), &mtc_value);

    // user ouput needs to be replaced with logging to journalctl and notification
    let notification_body = format!("likely Harddrive transplantation due to missmatch of MTC values - (current MTC): {} (Saved MTC) {} ", &current_mtc.to_string() , &old_mtc.to_string());
    if old_mtc > current_mtc {
        if let Err(err)  = send_notification(&notification_body){print!("{err}")}
    }
    let diff_mtc = current_mtc - old_mtc;
    if diff_mtc > 1{
        let notification_body = diff_mtc.to_string() +" times has your PC has been booted since your last login!";
        if let Err(err)  = send_notification(&notification_body){print!("{err}")}

    }
    Ok(())
}
