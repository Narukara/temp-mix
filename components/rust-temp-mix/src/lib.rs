#![no_std]

extern "C" {
    fn mycomp_test();
}

#[no_mangle]
extern "C" fn rust_main() -> i32 {
    unsafe {
        mycomp_test();
    }
    42
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
    }
}
