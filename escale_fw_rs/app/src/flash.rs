use core::mem::{size_of, MaybeUninit};

use rp2040_flash::flash;

const FLASH_ORIGIN: usize = 0x10000000;
/// The erasable sector size.
const FLASH_SECTOR_SIZE: usize = 4096;

/// The payload type `T` must fit into a single flash sector
/// and be `repr(C)` to have a stable layout.
///
/// This data type itself should be flash sector aligned.
#[repr(C, align(4096))]
pub union FlashSector<T>
where
    T: Copy,
    // `FLASH_SECTOR_SIZE` must be a power of two
    [(); 0 - !is_pwr_of_two(FLASH_SECTOR_SIZE) as usize]:,
    // `T` must fit into a single sector size
    [(); FLASH_SECTOR_SIZE - size_of::<T>()]:,
{
    data: [u8; FLASH_SECTOR_SIZE],
    value: MaybeUninit<T>,
}

impl<T> FlashSector<T>
where
    T: Copy,
    [(); FLASH_SECTOR_SIZE - size_of::<T>()]:,
{
    pub const fn uninit() -> Self {
        Self {
            value: MaybeUninit::uninit(),
        }
    }

    const fn new(value: T) -> Self {
        Self {
            value: MaybeUninit::new(value),
        }
    }

    fn mem_addr(&self) -> usize {
        unsafe { &self.data as *const _ as usize }
    }

    pub fn read(&self) -> MaybeUninit<T> {
        unsafe { self.value }
    }

    pub unsafe fn write(&mut self, value: T) {
        let tmp_flash_block = FlashSector::new(value);
        self.write_flash(&tmp_flash_block.data)
    }

    unsafe fn write_flash(&self, data: &[u8; FLASH_SECTOR_SIZE]) {
        let flash_addr = self.mem_addr() - FLASH_ORIGIN;
        assert!(is_aligned(flash_addr, FLASH_SECTOR_SIZE));

        cortex_m::interrupt::free(|_cs| {
            flash::flash_range_erase_and_program(flash_addr as u32, data, true);
        });
    }
}

const fn is_pwr_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1) == 0)
}

const fn is_aligned(addr: usize, alignment: usize) -> bool {
    assert!(is_pwr_of_two(alignment));
    addr & (alignment - 1) == 0
}
