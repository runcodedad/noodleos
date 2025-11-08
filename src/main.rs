#![no_std]
#![no_main]

use core::panic::PanicInfo;
use arch::{clear_screen, println, setup_idt, init_memory};

mod arch;

#[cfg(feature = "test-exceptions")]
mod tests;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point called by the bootloader
#[no_mangle]
pub extern "C" fn kernel_main(multiboot_info_addr: usize, multiboot_magic: usize) -> ! {
    // Clear the screen
    clear_screen();
    
    // Print our message - now we're truly in 64-bit long mode!
    println("Hello from NoodleOS - 64-bit Long Mode!");
    
    // Initialize the IDT
    setup_idt();
    println("IDT initialized successfully!");
    
    // Initialize memory subsystem
    init_memory(multiboot_info_addr, multiboot_magic);
    
    // Run tests if enabled via features
    #[cfg(feature = "test-exceptions")]
    {
        tests::run_all_tests();
    }
    
    println("Kernel initialization complete.");
    println("System ready. CPU will now halt.");
    
    // Halt the CPU - simple infinite loop
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
