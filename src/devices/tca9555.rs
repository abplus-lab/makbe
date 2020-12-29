// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;

pub struct TCA9555 {
    address: uint
}

impl Device for TCA9555 {
    fn pin_count(&self) -> usize {
        16
    }

    fn read_pins<T>(&self, mut pins: &[bool]) {
        unimplemented!()
    }
}