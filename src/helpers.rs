pub fn print_bytes(buffer: Vec<u8>) {
    let string_result = String::from_utf8(buffer.to_vec());
    if string_result.is_ok() {
        print!("{}", string_result.unwrap());
    } else {
        println!("Invalid UTF-8 data");
    }
}