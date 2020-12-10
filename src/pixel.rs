use crate::xil;
use crate::LED_ADDRESS;
// Use the print macros that are exported into crate root
use crate::{print64, print};

const PAGE_SIZE: usize = 10;

pub static mut PAGE: usize = 0;

/// Table for dots. Indices are page, x, y, color. Initialized to zero.
static mut DOTS: [[[[u8; PAGE_SIZE]; 8]; 8]; 3] = [[[[0; PAGE_SIZE]; 8]; 8]; 3];

pub unsafe fn setup_led_matrix() {
    // Tip: use the following to set an ADDRESS to zero:
    /*
    core::ptr::write_volatile(ADDRESS, 0);
    */

    // The screen must be reset at start
    // Tip: use the following one-liners to flip bits on or off at ADDRESS. Oh
    // yes, it's a zero-cost lambda function in an embedded application.
    /*
    mutate_ptr(ADDR, |x| x | 1);
    mutate_ptr(ADDR, |x| x ^ 1);
    */

    // TODO: Write code that sets 6-bit values in register of DM163 chip. It is
    // recommended that every bit in that register is set to 1. 6-bits and 24
    // "bytes", so some kind of loop structure could be nice
}

/// Set the value of one pixel at the LED matrix. Function is unsafe because it
/// uses global memory
unsafe fn set_pixel(x: usize, y: usize, r: u8, g: u8, b: u8) {
    // TODO: Set new pixel value. Take the parameeters and put them into the
    // DOTS array.
}

/// Refresh new data into the LED matrix. Hint: This function is supposed to
/// send 24-bytes and parameter x is for x-coordinate.
pub unsafe fn run(c: usize) {
    // TODO: Write into the LED matrix driver (8-bit data). Use values from DOTS
    // array.
}

/// Latch signal for the colors shield. See colorsshield.pdf for how latching
/// works.
unsafe fn latch() {
    // TODO: Do the latching operation
}

/// Sets one line, matching with the parameter, as active.
pub unsafe fn open_line(i: u8) {
    // TODO: Write code here.
    // Tip: use a `match` statement for the parameter:
    /*
    match i {
        0 => {},
        _ => {},
    }
    */
}

/// A helper one-liner for mutating raw pointers at given address. You shouldn't need to change this.
unsafe fn mutate_ptr<A, F>(addr: *mut A, mutate_fn: F)
where
    F: FnOnce(A) -> A,
{
    let prev = core::ptr::read_volatile(addr);
    let new = mutate_fn(prev);
    core::ptr::write_volatile(addr, new);
}
