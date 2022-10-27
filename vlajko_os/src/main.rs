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
  use vlajko_os::memory;
  use x86_64::{structures::paging::Translate, VirtAddr};

  println!("Hello World!");
  vlajko_os::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mapper = unsafe { memory::init(phys_mem_offset) };

  let addresses = [
      // vga buffer page
      0xb8000,
      // part of code segment
      0x201008,
      // part of stack segment
      0x0100_0020_1a10,
      // virtual address mapped to physical address 0
      boot_info.physical_memory_offset,
  ];

  for &address in &addresses {
      let virt =  VirtAddr::new(address);
      let phys = mapper.translate_addr(virt);
      println!("{:?} -> {:?}", virt, phys);
  }

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
