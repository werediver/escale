use core::{
    mem::{size_of, MaybeUninit},
    ptr::read_volatile,
};

use rp2040_flash::flash;

pub const FLASH_ORIGIN: usize = 0x10000000;
const FLASH_SECTOR_SIZE: usize = 4096;
const FLASH_ERASED_VALUE: u8 = 0xff;

pub union Flash<T>
where
    T: Copy,
    [(); sector_aligned_size_of::<T>()]:,
{
    data: [u8; sector_aligned_size_of::<T>()],
    value: MaybeUninit<T>,
}

impl<T> Default for Flash<T>
where
    T: Copy,
    [(); sector_aligned_size_of::<T>()]:,
{
    fn default() -> Self {
        Self {
            data: [FLASH_ERASED_VALUE; sector_aligned_size_of::<T>()],
        }
    }
}

impl<T> Flash<T>
where
    T: Copy,
    [(); sector_aligned_size_of::<T>()]:,
{
    pub fn new(value: T) -> Self {
        let mut instance = Self::default();
        instance.value = MaybeUninit::new(value);
        instance
    }

    pub fn read(mem_addr: usize) -> Self {
        let mut flash_sector = Flash::default();
        flash_sector.value = unsafe { read_volatile(mem_addr as *const _) };
        flash_sector
    }

    pub unsafe fn write(&self, mem_addr: usize) {
        let flash_addr = mem_addr - FLASH_ORIGIN;
        assert!(is_aligned(flash_addr, FLASH_SECTOR_SIZE));

        flash::flash_range_erase_and_program(flash_addr as u32, &self.data, true);
    }

    pub fn value(&self) -> MaybeUninit<T> {
        unsafe { self.value }
    }
}

pub const fn sector_aligned_size_of<T>() -> usize {
    assert!(is_pwr_of_two(FLASH_SECTOR_SIZE));
    ceil_step(size_of::<T>(), FLASH_SECTOR_SIZE)
}

const fn ceil_step(value: usize, step: usize) -> usize {
    let value_floor = value / step * step;
    let r = value - value_floor;
    if r > 0 {
        value_floor + step
    } else {
        value_floor
    }
}

const fn is_aligned(addr: usize, alignment: usize) -> bool {
    assert!(is_pwr_of_two(alignment));
    addr & (alignment - 1) == 0
}

const fn is_pwr_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1) == 0)
}
