// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

/// I2Cにぶら下がったデバイス（大抵の場合、I/Oエキスパンダ）
pub trait Device {

    /// 扱うPIN数を返す
    fn pin_count(&self) -> usize;

    /// PINの状態を読み込んで、pinsスライスを更新する
    fn read_pins<T>(&self, mut pins: &[bool] );
}
