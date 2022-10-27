#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(vlajko_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use vlajko_os::println;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use vlajko_os::memory::BootInfoFrameAllocator;
  use vlajko_os::memory;
  use x86_64::{structures::paging::Translate, VirtAddr};
  use x86_64::{structures::paging::Page};

  println!("Hello World!");
  vlajko_os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

  #[cfg(test)]
  test_main();

  vlajko_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    vlajko_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vlajko_os::test_panic_handler(info)
}
