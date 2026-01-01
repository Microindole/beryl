#[no_mangle]
pub extern "C" fn __beryl_panic_bounds(index: i64, size: i64) -> ! {
    eprintln!("Runtime Error: Array index out of bounds.");
    eprintln!("  Index: {}", index);
    eprintln!("  Array size: {}", size);
    std::process::exit(1);
}
