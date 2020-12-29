// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use keyberon::action::Action;

pub trait Binding {

    /// Onになっているpinのアクションをスライスで返す
    fn to_action(&self, pins: &[bool]) -> &[Action];
}
