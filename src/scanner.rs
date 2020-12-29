// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

use crate::device::Device;
use crate::talker::Talker;

/// deviceを使用して、キーの状態をスキャンするもの
pub trait Scanner {
    /// 全体のキーをスキャンして、その状態（押されていればtrue）を返す
    fn scan(&self) -> &[bool];
}

