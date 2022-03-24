//! Implementation of `SizeReport` for `Vec` and slice types.



use crate::SizeReport;



impl<T: SizeReport> SizeReport for Vec<T> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.capacity() * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}

impl<T: SizeReport> SizeReport for &[T] {
    const ALLOC: bool = false;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.len() * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}

impl<T: SizeReport> SizeReport for &mut [T] {
    const ALLOC: bool = false;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.len() * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}

impl<T: SizeReport, const N: usize> SizeReport for &[T; N] {
    const ALLOC: bool = false;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        N * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}

impl<T: SizeReport, const N: usize> SizeReport for &mut [T; N] {
    const ALLOC: bool = false;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        N * T::direct()
    }

    fn children(&self) -> usize {
        if !(T::ALLOC || T::CHILD) { return 0; }

        self.iter().map(|x| x.indirect() + x.children()).sum()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct() {
        assert!((core::mem::size_of::<usize>() * 3) == Vec::<u64>::direct());
        assert!((core::mem::size_of::<usize>() * 2) == <&[u8]>::direct());
        assert!((core::mem::size_of::<usize>()    ) == <&[u8; 2]>::direct());
    }

    #[test]
    fn indirect() {
        assert!((core::mem::size_of::<usize>() * 4) == Vec::<u64>::with_capacity(4).indirect());

        let array = [0u8, 1, 2, 3];
        assert!((core::mem::size_of::<u8>() * 4) == (&array).indirect());
        assert!((core::mem::size_of::<u8>() * 4) == (&array[..]).indirect());
    }

    #[test]
    fn children() {
        assert!(0 == vec![0usize, 1, 2, 3].children());

        let array = [0u8, 1, 2, 3];

        assert!(0 == (&array).children());
        assert!(0 == (&array[..]).children());
    }
}
