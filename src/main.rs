#![no_std] //  don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

 
use core::panic::PanicInfo;
use os::println;


// static HELLO: &[u8] = b"Hello World!";
/* Disable name mangling to 
ensure the rust compiler really outputs a function with the name _start*/
#[no_mangle]
pub extern "C" fn _start() -> ! { // !  because instaed of returning it should envoke exit syscall
    // This function is the entry point, since thel inker looks for a function
    // named `_start` by default
     println!("hello World{}", "1");

     os::init();


     // invoke a preakpoint exception
     x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

 /// This function is called on panic.
#[cfg(not(test))]
#[panic_handler] // defines what the compiler should envoke on a panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


