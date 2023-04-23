#[no_mangle]
pub extern "C" fn handle(x: i32) -> i32 {
    x * 2
}

