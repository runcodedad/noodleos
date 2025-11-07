#![no_std]
#![no_main]

use core::panic::PanicInfo;
use arch::{clear_screen, println, setup_idt};

mod arch;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point called by the bootloader
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Clear the screen
    clear_screen();
    
    // Print our message - now we're truly in 64-bit long mode!
    println("Hello from NoodleOS - 64-bit Long Mode!");
    
    // Initialize the IDT
    setup_idt();
    println("IDT initialized successfully!");
    
    // Halt the CPU - simple infinite loop
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
