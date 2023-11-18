#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

const RCC_ADDR: u32 = 0x4002_1000;
const RCC_AHBENR_OFFSET: u32 = 0x14;
const RCC_AHBENR: u32 = RCC_ADDR + RCC_AHBENR_OFFSET;

const GPIOE_ADDR: u32 = 0x4800_1000;
const GPIO_BSRR_OFFSET: u32 = 0x18;
const GPIOE_BSRR_ADDR: u32 = GPIOE_ADDR + GPIO_BSRR_OFFSET;
const GPIO_MODER_OFFSET: u32 = 0x00;
const GPIOE_MODER_ADDR: u32 = GPIOE_ADDR + GPIO_MODER_OFFSET;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!!").unwrap();

    unsafe {
        // Enable the GPIOE peripheral
        let ahbenr = &*(RCC_AHBENR as *mut volatile_register::RW<u32>);
        ahbenr.modify(|r| r | (1 << 21));

        // Set pin as output
        let moder = &*(GPIOE_MODER_ADDR as *mut volatile_register::RW<u32>);

        let output_pin = 14;

        let pin_shift = output_pin * 2; // Calculate the bit position based on pin number
        let mask = 0b11 << pin_shift; // Create a mask for the pin bits in the register

        let mode = 0b01; // General purpose output mode
        let set_mode = mode << pin_shift; // Shift the mode to the correct position

        moder.modify(|r| (r & !mask) | set_mode); // First clear the two bits of this pins mode, then OR with the new (bit-shifted) value
        let bsrr = &*(GPIOE_BSRR_ADDR as *mut volatile_register::RW<u32>);
        bsrr.write(1 << output_pin); // A pin is set by setting the bit in the lower 16 bits of the BSRR
    }

    loop {
        // your code goes here
    }
}
