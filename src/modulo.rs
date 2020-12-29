// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::scanner::Scanner;
use crate::device::Device;
use crate::talker::Talker;

pub struct ModuloScanner {
    devices: &'static[dyn Device],
    talker: &'static dyn Talker
}

impl ModuloScanner {

    pub const fn new(devices: &'static[dyn Device], talker: &'static dyn Talker) -> Self {
        Self {
            devices,
            talker
        }
    }
}

impl Scanner for ModuloScanner {

    fn scan(&self) -> &[bool] {
        let mut offset = 0;
        let size = self.devices.iter().fold(0: usize, |a, d| a + d.pin_count());
        let buffer = Vec::<bool>::with_capacity(size).as_mut_slice();
        for dev in self.devices.iter() {
            let count = dev.pin_count();
            let tail = offset + count;
            let rins = &buffer[offset..tail];
            offset +=  count;
        }
        &buffer
    }
}
