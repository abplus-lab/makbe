// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use keyberon::action::Action;

pub trait Binding {

    /// OnになっているpinのアクションあるいはNoneをスライスで返す
    fn to_action(&self, pins: &[bool]) -> &[Option<Action>];
}
