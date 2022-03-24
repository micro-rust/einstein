//! Implementation of `SizeReport` for `HashMap`.



use crate::SizeReport;
use std::collections::HashMap;



impl<K: SizeReport, V: SizeReport> SizeReport for HashMap<K, V> {
    const ALLOC: bool = true;
    const CHILD: bool = true;

    fn indirect(&self) -> usize {
        self.capacity() * (K::direct() + V::direct())
    }

    fn children(&self) -> usize {
        let k = if K::ALLOC || K::CHILD {
            self.keys().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        };

        let v = if V::ALLOC || V::CHILD {
            self.values().map(|x| x.indirect() + x.children()).sum()
        } else {
            0
        };

        k + v
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn direct() {
        assert_eq!((core::mem::size_of::<usize>() * 6), HashMap::<u64, u64>::direct());
        assert_eq!((core::mem::size_of::<usize>() * 6), HashMap::<String, String>::direct());
    }

    #[test]
    fn indirect() {
        let map: HashMap<String, String> = HashMap::with_capacity(4);

        assert_eq!(map.capacity() * String::direct() * 2, map.indirect());
    }

    #[test]
    fn children() {
        let mut map: HashMap<String, String> = HashMap::with_capacity(4);

        let a = String::from("TESTA");
        let b = String::from("TESTB");
        let c = String::from("TESTC");
        let d = String::from("TESTD");

        map.insert(a.clone(), a.clone());
        map.insert(b.clone(), b.clone());
        map.insert(c.clone(), c.clone());
        map.insert(d.clone(), d.clone());

        assert_eq!(a.indirect() * 4 * 2, map.children());
    }
}
