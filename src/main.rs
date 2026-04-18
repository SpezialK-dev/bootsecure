use std::fs;
use std::fs::File;
use std::io::Write;
use std::{io::Read};
use std::fs::OpenOptions;


const BASE_PATH: &'static str =  "/var/lib/bootsecure/";
const BASE_CONFIG: &'static str =  "/var/lib/bootsecure/save_mtc";


// all paarsing for monotonic counter
fn correct_uefi_var(file_name : &String) -> bool{
    let mtc_vars = ["MTC", "MonotonicCounter"];
    for poss_var in mtc_vars{
        if file_name.contains(poss_var){
            return true;
        }
    }
    false

}

#[cfg(target_os="linux")]
fn read_MTC(value :&mut u16) -> std::io::Result<()>{

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
    *value = u16::try_from(0).unwrap();
    Ok(())
}
//storing and and restoring the value to a config file
//we want the value to be consumed when writing since its the last action being done
fn write_value_to_config(data: String)-> std::io::Result<()>{
    fs::create_dir_all(BASE_PATH)?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(BASE_CONFIG)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn read_value_from_mtc_val(value :&mut u16)->std::io::Result<()>{
    let exists = match fs::exists(BASE_CONFIG) {
        Ok(val) => val,
        Err(val) => false // if something went wrong the file might as well not exist
    };
    if exists{
        let content_mtc = match fs::read_to_string(BASE_CONFIG){
            Ok(val) => val,
            Err(err) => {print!("The following error occued while reading {err}");String::from("0")} //if 0 is returned from config file
        };
        *value = match content_mtc.parse::<u16>(){
            Ok(val) => val,
            Err(err) => {print!("The following Error occured while casting: {err}"); 0}
        }
    }else {
        *value = u16::try_from(0).unwrap()
    }
    Ok(())
}

fn main() {
    let mut old_mtc : u16 = 0;
    let mut current_mtc : u16 = 0;
    //TODO better error handling here
    let _ = read_value_from_mtc_val(&mut old_mtc);
    let _ = read_MTC(&mut current_mtc);
    let _ =write_value_to_config(current_mtc.to_string());
    // user ouput needs to be replaced with logging to journalctl and notification
    if(old_mtc > current_mtc){
        print!("saved MTC: {old_mtc}\ncurrent MTC: {current_mtc}\n You likely moved plattfer between PCs Otherwiese attack has happend!")
    }
    let diff_mtc = current_mtc - old_mtc;
    if (diff_mtc > 1){
        print!("{diff_mtc} Boots have happend since your last login!!")
    }
}
