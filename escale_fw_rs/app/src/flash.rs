use core::{
    mem::{size_of, MaybeUninit},
    ptr::read_volatile,
};

use rp2040_flash::flash;

#[allow(non_upper_case_globals)]
const MiB: usize = 1024 * 1024;

/// XIP base address (see `XIP_BASE` in RP2040 datasheet).
pub const FLASH_ORIGIN: usize = 0x10000000;
/// RP2040 supports maximum 16 MiB of QSPI flash memory.
pub const FLASH_END_MAX: usize = FLASH_ORIGIN + 16 * MiB;
/// The erasable sector size.
const FLASH_SECTOR_SIZE: usize = 4096;
/// The value an erased sector is filled with. This is typically 0xff.
const FLASH_ERASED_VALUE: u8 = 0xff;

/// The payload type should be `repr(C)` to have a stable layout,
/// because the flash-stored payload can survive firmware upgrades.
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

    pub unsafe fn read(mem_addr: usize) -> Self {
        assert!(mem_addr >= FLASH_ORIGIN);
        assert!(mem_addr <= FLASH_END_MAX - size_of::<Self>());
        // The read address must be sector-aligned, because the write function
        // only ever allows writing at sector-aligned addresses.
        assert!(is_aligned(mem_addr, FLASH_SECTOR_SIZE));

        let mut flash_sector = Flash::default();
        flash_sector.value = unsafe { read_volatile(mem_addr as *const _) };
        flash_sector
    }

    pub unsafe fn write(&self, mem_addr: usize) {
        assert!(mem_addr >= FLASH_ORIGIN);
        assert!(mem_addr <= FLASH_END_MAX - size_of::<Self>());
        assert!(is_aligned(mem_addr, FLASH_SECTOR_SIZE));

        let flash_addr = mem_addr - FLASH_ORIGIN;
        flash::flash_range_erase_and_program(flash_addr as u32, &self.data, true);
    }

    pub fn value(&self) -> MaybeUninit<T> {
        unsafe { self.value }
    }
}

pub const fn sector_aligned_size_of<T>() -> usize {
    assert!(FLASH_SECTOR_SIZE.is_power_of_two());
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
    assert!(alignment.is_power_of_two());
    addr & (alignment - 1) == 0
}
