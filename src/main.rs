fn main() {
    match mac_address::get_mac_address() {
        Ok(Some(mac)) => println!("mac = {:?}", mac.to_string()),
        Ok(None) => println!("no mac"),
        Err(e) => println!("error = {:?}", e),
    }
}
