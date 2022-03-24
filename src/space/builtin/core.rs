//! Implementation of `SizeReport` for all basic types.



use crate::SizeReport;



impl SizeReport for i8   {}
impl SizeReport for i16  {}
impl SizeReport for i32  {}
impl SizeReport for i64  {}
impl SizeReport for i128 {}

impl SizeReport for u8   {}
impl SizeReport for u16  {}
impl SizeReport for u32  {}
impl SizeReport for u64  {}
impl SizeReport for u128 {}

impl SizeReport for isize {}
impl SizeReport for usize {}

impl SizeReport for f32 {}
impl SizeReport for f64 {}

impl SizeReport for bool {}
impl SizeReport for char {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct() {
        assert!(core::mem::size_of::<u8>()   == u8::direct());
        assert!(core::mem::size_of::<u16>()  == u16::direct());
        assert!(core::mem::size_of::<u32>()  == u32::direct());
        assert!(core::mem::size_of::<u64>()  == u64::direct());
        assert!(core::mem::size_of::<u128>() == u128::direct());

        assert!(core::mem::size_of::<i8>()   == i8::direct());
        assert!(core::mem::size_of::<i16>()  == i16::direct());
        assert!(core::mem::size_of::<i32>()  == i32::direct());
        assert!(core::mem::size_of::<i64>()  == i64::direct());
        assert!(core::mem::size_of::<i128>() == i128::direct());

        assert!(core::mem::size_of::<isize>() == isize::direct());
        assert!(core::mem::size_of::<usize>() == usize::direct());

        assert!(core::mem::size_of::<f32>() == f32::direct());
        assert!(core::mem::size_of::<f64>() == f64::direct());

        assert!(core::mem::size_of::<char>() == char::direct());
        assert!(core::mem::size_of::<bool>() == bool::direct());
    }

    #[test]
    fn indirect() {
        assert!(0 == 0u8.indirect());
        assert!(0 == 0u16.indirect());
        assert!(0 == 0u32.indirect());
        assert!(0 == 0u64.indirect());
        assert!(0 == 0u128.indirect());

        assert!(0 == 0i8.indirect());
        assert!(0 == 0i16.indirect());
        assert!(0 == 0i32.indirect());
        assert!(0 == 0i64.indirect());
        assert!(0 == 0i128.indirect());

        assert!(0 == 0isize.indirect());
        assert!(0 == 0usize.indirect());

        assert!(0 == 0.0f32.indirect());
        assert!(0 == 0.0f64.indirect());

        assert!(0 == 'a'.indirect());
        assert!(0 == true.indirect());
    }

    #[test]
    fn children() {
        assert!(0 == 0u8.children());
        assert!(0 == 0u16.children());
        assert!(0 == 0u32.children());
        assert!(0 == 0u64.children());
        assert!(0 == 0u128.children());

        assert!(0 == 0i8.children());
        assert!(0 == 0i16.children());
        assert!(0 == 0i32.children());
        assert!(0 == 0i64.children());
        assert!(0 == 0i128.children());

        assert!(0 == 0isize.children());
        assert!(0 == 0usize.children());

        assert!(0 == 0.0f32.children());
        assert!(0 == 0.0f64.children());

        assert!(0 == 'a'.children());
        assert!(0 == true.children());
    }
}
