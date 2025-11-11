/// VGA text buffer memory address
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

/// Current cursor position (static for simple implementation)
static mut CURSOR_POS: usize = 0;

/// Clear the screen with black background
pub fn clear_screen() {
    let vga_buffer = VGA_BUFFER;
    unsafe {
        CURSOR_POS = 0; // Reset cursor position
        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT * 2) {
            if i % 2 == 0 {
                *vga_buffer.add(i) = b' '; // space character
            } else {
                *vga_buffer.add(i) = 0x07; // white text on black background
            }
        }
    }
}

/// Scroll the screen up by one line
fn scroll_up() {
    let vga_buffer = VGA_BUFFER;
    unsafe {
        // Copy each line to the previous line
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let src = (row * BUFFER_WIDTH + col) * 2;
                let dst = ((row - 1) * BUFFER_WIDTH + col) * 2;
                *vga_buffer.add(dst) = *vga_buffer.add(src);         // character
                *vga_buffer.add(dst + 1) = *vga_buffer.add(src + 1); // color
            }
        }
        
        // Clear the last line
        let last_row = BUFFER_HEIGHT - 1;
        for col in 0..BUFFER_WIDTH {
            let offset = (last_row * BUFFER_WIDTH + col) * 2;
            *vga_buffer.add(offset) = b' ';     // space
            *vga_buffer.add(offset + 1) = 0x0F; // white on black
        }
    }
}

/// Print a string at the current cursor position
pub fn print_string(message: &str) {
    let vga_buffer = VGA_BUFFER;
    
    unsafe {
        for &byte in message.as_bytes().iter() {
            if byte == b'\n' {
                // Move to next line
                CURSOR_POS = ((CURSOR_POS / BUFFER_WIDTH) + 1) * BUFFER_WIDTH;
            } else {
                // Check if we need to scroll
                if CURSOR_POS >= BUFFER_WIDTH * BUFFER_HEIGHT {
                    scroll_up();
                    CURSOR_POS = (BUFFER_HEIGHT - 1) * BUFFER_WIDTH;
                }
                
                let offset = CURSOR_POS * 2;
                *vga_buffer.add(offset) = byte;        // character
                *vga_buffer.add(offset + 1) = 0x0F;    // white on black
                CURSOR_POS += 1;
            }
        }
    }
}

/// Print a string without newline
pub fn print(message: &str) {
    print_string(message);
}

/// Print a string and move to the next line
pub fn println(message: &str) {
    print_string(message);
    print_string("\n");
}
