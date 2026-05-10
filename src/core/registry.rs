use windows_sys::Win32::Foundation::{ERROR_SUCCESS, GetLastError};
use windows_sys::Win32::System::Registry::{HKEY, KEY_READ, RegOpenKeyExW, RegQueryValueExW, RegQueryInfoKeyW};

type DWORD = u32;


use crate::core::utils::to_wide_str;

pub fn reg_open_key_exw(hive: HKEY, subkey: &str) -> Result<HKEY, u32> {
    let subkey_wide = to_wide_str(subkey);
    let mut hkey: HKEY = std::ptr::null_mut();

    let result = unsafe { RegOpenKeyExW(hive, subkey_wide.as_ptr(), 0, KEY_READ, &mut hkey) };

    if result != ERROR_SUCCESS {
        return Err(unsafe { GetLastError() });
    }

    Ok(hkey)
}

pub fn reg_query_value_exw(hive: HKEY, subkey: &str, data_size: usize) -> Result<Vec<u8>, u32> {
    let subkey_wide = to_wide_str(subkey);
    let mut size = data_size as u32;

    let mut buffer: Vec<u8> = vec![0u8; data_size];

    let result = unsafe {
        RegQueryValueExW(
            hive,
            subkey_wide.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            buffer.as_mut_ptr(),
            &mut size,
        )
    };

    if result != ERROR_SUCCESS {
        return Err(unsafe { GetLastError() });
    }

    Ok(buffer)
}

pub fn reg_query_value_exw_dword(hive: HKEY, subkey: &str) -> Result<DWORD, u32> {
    let data_size = size_of::<DWORD>();

    let data = reg_query_value_exw(hive, subkey, data_size)?;

    let mut array = [0u8; 4];
    array.copy_from_slice(&data);

    Ok(DWORD::from_le_bytes(array))
}

pub fn reg_query_info_w(hive: HKEY, data_size: usize) -> Result<Vec<u16>, u32> {
    let mut size = data_size as u32;

    let mut buffer: Vec<u16> = vec![0u16; data_size];

    let result = unsafe {
        RegQueryInfoKeyW(
            hive,
            buffer.as_mut_ptr(),
            &mut size,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if result != ERROR_SUCCESS {
        return Err(unsafe { GetLastError() });
    }

    Ok(buffer)
}
