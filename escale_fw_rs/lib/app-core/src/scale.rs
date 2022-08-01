use num_traits::{float::FloatCore, PrimInt};
use stuff::{ring::Ring, signal::mean};

pub struct Scale<T: PrimInt, U: FloatCore, const N: usize>
where
    // Require N ≥ 1
    [(); N - 1]:,
{
    ring: Ring<T, N>,
    tare: U,
    unit: U,
}

impl<T: PrimInt + Default, U: FloatCore, const N: usize> Default for Scale<T, U, N>
where
    [(); N - 1]:,
{
    fn default() -> Self {
        Self {
            ring: Default::default(),
            unit: U::one(),
            tare: U::zero(),
        }
    }
}

impl<T: PrimInt, U: FloatCore, const N: usize> Scale<T, U, N>
where
    [(); N - 1]:,
{
    pub fn push(&mut self, value: T) {
        self.ring.push(value);
    }

    pub fn is_filled(&self) -> bool {
        self.ring.is_filled()
    }

    pub fn reset(&mut self) {
        self.ring.reset(T::zero())
    }

    /// Set the zero offset (tare) based on the current buffer.
    ///
    /// The buffer must be filled.
    pub fn set_tare(&mut self) -> Result<(), Error> {
        if self.is_filled() {
            self.tare = self.read_raw()?;
            Ok(())
        } else {
            Err(Error::NotFilled)
        }
    }

    /// Set the calibration coefficient based on the current buffer.
    ///
    /// `value` allows to set the unit to a fraction of the current readout.
    ///
    /// If the desired unit is 1 g and a 100 g weight is used for calibration,
    /// set `value` to 100.
    pub fn set_unit(&mut self, value: U) -> Result<(), Error> {
        assert!(value != U::zero());
        if self.is_filled() {
            let unit = (self.read_raw()? - self.tare) / value;
            assert!(unit != U::zero());
            self.unit = unit;
            Ok(())
        } else {
            Err(Error::NotFilled)
        }
    }

    fn read_raw(&self) -> Result<U, Error> {
        if self.is_filled() {
            Ok(mean(self.ring.iter().copied()).unwrap())
        } else {
            Err(Error::NotFilled)
        }
    }

    pub fn read(&self) -> Result<U, Error> {
        if self.is_filled() {
            Ok(mean(self.ring.iter().copied().map(|x| self.adjust(x))).unwrap())
        } else {
            Err(Error::NotFilled)
        }
    }

    fn adjust(&self, raw: T) -> U {
        const E_RAW_MUST_FIT: &str = &"Raw readout must fit into the output floating point type";
        (U::from(raw).expect(E_RAW_MUST_FIT) - self.tare) / self.unit
    }
}

#[derive(Debug)]
pub enum Error {
    NotFilled,
}
