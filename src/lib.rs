// Copyright 2020 Kazuyuki HIDA <kazhida@abplus.com>
// All right reserved.
//

mod scanner;
mod device;
mod talker;
mod devices;
mod binding;
mod keyboard;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
