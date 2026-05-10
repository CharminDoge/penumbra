use crate::core;

use windows_sys::Win32::System::Registry::{HKEY, HKEY_LOCAL_MACHINE};

const SYSKEY_LENGTH: usize = 16;

pub fn get_sys_key() -> Result<[u8; SYSKEY_LENGTH], u32> {
    let system_base = core::registry::reg_open_key_exw(HKEY_LOCAL_MACHINE, "SYSTEM")?;

    let control_set = get_current_control_set(system_base)?;

    let control_lsa = core::registry::reg_open_key_exw(control_set, "Control\\LSA")?;

    read_sys_key(control_lsa)
}

const SYSKEY_INFO_LENGTH: u32 = 8 + 1;
const SYSKEY_NAMES: [&str; 4] = ["JD", "Skew1", "GBG", "Data"];
const SYSKEY_PERMUT: [u8; 16] = [7, 3, 10, 8, 15, 9, 1, 2, 4, 13, 5, 0, 14, 12, 6, 11];

pub fn read_sys_key(control_lsa: HKEY) -> Result<[u8; SYSKEY_LENGTH], u32> {
    let mut sys_key: [u8; SYSKEY_LENGTH] = [0u8; SYSKEY_LENGTH];
    for (i, syskey_name) in SYSKEY_NAMES.iter().enumerate() {
        match core::registry::reg_open_key_exw(control_lsa, syskey_name) {
            Ok(lsa_key) => {
                let data = core::registry::reg_query_info_w(lsa_key, SYSKEY_INFO_LENGTH as usize)?;

                let hex_data = parse_hex_utf16(&data);
                let bytes = hex_data.to_le_bytes();

                for j in 0..4 {
                    sys_key[SYSKEY_PERMUT[i * 4 + j] as usize] = bytes[j]
                }
            }
            Err(e) => {
                println!(
                    "[!] reg_open_key_exw on {} failed with error: {}",
                    syskey_name, e
                );

                return Err(e);
            }
        };
    }

    Ok(sys_key)
}

fn parse_hex_utf16(buf: &[u16]) -> u32 {
    const C0: u16 = b'0' as u16;
    const C9: u16 = b'9' as u16;
    const CA: u16 = b'A' as u16;
    const CF: u16 = b'F' as u16;
    const CA_L: u16 = b'a' as u16;
    const CF_L: u16 = b'f' as u16;

    let mut value: u32 = 0;
    for &c in buf {
        if c == 0 {
            break;
        }
        let digit = match c {
            C0..=C9 => c - C0,
            CA_L..=CF_L => c - CA_L + 10,
            CA..=CF => c - CA + 10,
            _ => break,
        };
        value = value * 16 + digit as u32;
    }
    value
}

const CONTROLSET_SOURCES: [&str; 2] = ["Current", "Default"];

pub fn get_current_control_set(system_base: HKEY) -> Result<HKEY, u32> {
    let select = core::registry::reg_open_key_exw(system_base, "Select")?;

    let mut last_err = 0;
    for source in CONTROLSET_SOURCES {
        match core::registry::reg_query_value_exw_dword(select, source) {
            Ok(control_set) => {
                let current_control_set = format!("ControlSet{:03}", control_set);
                return core::registry::reg_open_key_exw(system_base, &current_control_set);
            }
            Err(e) => {
                println!(
                    "[!] get_current_control_set: failed to retrieve {} with error: {}",
                    source, e
                );
                last_err = e;
            }
        };
    }

    Err(last_err)
}
