//! `SizeReport` is a trait to report the actual size of a struct, not
//! just the pointer in the stack, but the whole size including heap allocated
//! data and derivatives.



mod builtin;



pub trait SizeReport: Sized {
    /// A constant that indicates whether this object allocates memory other
    /// than the innate memory.
    const ALLOC: bool = false;

    /// A constant that indicates whether this object has children.
    const CHILD: bool = false;

    /// Reports the full amount of memory occupied by this struct and its children.
    #[inline]
    fn fullsize(&self) -> usize {
    	Self::direct() + self.indirect() + self.children()
    }

    /// Reports the memory innate to the struct.
    /// This is equivalent to `core::mem::size_of::<Self>()`.
    #[inline]
    fn direct() -> usize {
        core::mem::size_of::<Self>()
    }

    /// Reports the memory directly allocated by this struct.
    /// For example, `Vec` returns its capacity times the size of the structs.
    #[inline]
    fn indirect(&self) -> usize {
        0
    }

    /// Reports the indirect and children memory of the children of the struct,
    /// if there are any. This returns non zero only if the children of the
    /// struct allocate memory.
    /// For example `Vec<Vec<T>>` would return a non zero value but a
    /// `Vec<usize>` would return 0.
    #[inline]
    fn children(&self) -> usize {
        0
    }
}
