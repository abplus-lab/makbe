// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use keyberon::action::Action;

pub trait Talker {
    fn push(action: Action);
}