// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::scanner::Scanner;
use crate::binding::Binding;
use crate::talker::Talker;

struct Keyboard {
    scanner: &'static dyn Scanner,
    binding: &'static dyn Binding,
    talker: &'static dyn Talker
}

impl Keyboard {

    pub fn scan(&self) {
        let pins = self.scanner.scan();
        for a in self.binding.to_action(pins).iter() {
            match a {
                Some(action) => self.talker.push(a),
                None => {}
            }
        }
    }
}
