use std::mem;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::synchapi::WaitForSingleObject;
use winapi::um::memoryapi::{VirtualAlloc, VirtualProtect};
use winapi::um::processthreadsapi::{CreateThread, GetCurrentProcessId};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE};

pub fn get_current_pid() -> u32 {
    unsafe { GetCurrentProcessId() }
}

pub fn close_handle(handle: *mut c_void) -> Result<(), u32> {
    let result = unsafe { CloseHandle(handle) };

    if result == 0 {
        return Err(unsafe { GetLastError() });
    }

    Ok(())
}

pub fn virtual_alloc_rw(size: usize) -> Result<*mut u8, u32> {
    let address =
        unsafe { VirtualAlloc(null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE) };

    if address.is_null() {
        return Err(unsafe { GetLastError() });
    }

    Ok(address as *mut u8)
}

pub fn virtual_protect_erw(address: *mut u8, size: usize) -> Result<u32, u32> {
    let mut old_protection: u32 = 0;

    let result = unsafe {
        VirtualProtect(
            address as *mut _,
            size,
            PAGE_EXECUTE_READWRITE,
            &mut old_protection,
        )
    };

    if result == 0 {
        return Err(unsafe { GetLastError() });
    }

    Ok(old_protection)
}

pub fn create_thread(address: *mut u8) -> Result<*mut c_void, u32> {
    let thread_handle = unsafe {
        CreateThread(
            null_mut(),
            0,
            Some(mem::transmute(address)),
            null_mut(),
            0,
            null_mut(),
        )
    };

    if thread_handle.is_null() {
        return Err(unsafe { GetLastError() });
    }

    Ok(thread_handle)
}

pub fn wait_for_single_object(handle: *mut c_void, milliseconds: u32) -> Result<(), u32> {
    let result = unsafe { WaitForSingleObject(handle, milliseconds) };

    if result == 0xFFFFFFFF {
        return Err(unsafe { GetLastError() });
    }

    Ok(())
}