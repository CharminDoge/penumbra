use penumbra::modules;

fn main() {
    let sys_key = match modules::lsa::syskey::get_sys_key() {
        Ok(sys_key) => sys_key,
        Err(e) => {
            println!("[!] get_sys_key failed with error: {}", e);
            return;
        }
    };

    println!("sys_key: {:02x?}", sys_key);
}
