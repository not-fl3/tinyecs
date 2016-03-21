use std::ptr;
use std::slice::*;

pub struct FastDictionary<T> {
    pub vec : Vec<Option<T>>
}
impl<T> FastDictionary<T> {
    pub fn new(max_index : i32) -> FastDictionary<T> {
        FastDictionary {
            vec : {
                let mut vec = vec![];
                vec.extend((0..max_index).map(|_| None));
                vec
            }
        }
    }

    pub fn insert(&mut self, key : usize, value : T) {
        if key >= self.vec.len() {
            let len = self.vec.len();
            let reserve = key - len + 1;
            self.vec.extend((0..reserve).map(|_| None));
        }
        unsafe {
            ptr::write(self.vec.as_mut_ptr().offset(key as isize), Some(value))
        }
    }

    #[inline]
    pub fn insert_no_check(&mut self, key : usize, value : T) {
        self.vec[key as usize] = Some(value);
    }
    #[inline]
    pub fn get_no_check(&self, key: isize) -> &T {
        unsafe {&(*self.vec.as_ptr().offset(key as isize)).as_ref().unwrap()}
    }

    #[inline]
    pub fn get_mut_no_check<'a>(&mut self, key: isize) -> &'a mut T {
        let ptr = self.vec.as_mut_ptr();
        unsafe {
            use std::slice::from_raw_parts_mut;

            from_raw_parts_mut(ptr.offset(key as isize), 1).get_mut(0).
                unwrap().as_mut().unwrap()
        }
    }


    #[inline]
    pub fn get2_mut_no_check(&mut self, key: isize, key2 : isize) ->
        (&mut T, &mut T) {
        let ptr = self.vec.as_mut_ptr();
        unsafe {
            use std::slice::from_raw_parts_mut;

            (from_raw_parts_mut(ptr.offset(key as isize), 1).get_mut(0).unwrap().as_mut().unwrap(),
            from_raw_parts_mut(ptr.offset(key2 as isize), 1).get_mut(0).unwrap().as_mut().unwrap())
        }
    }


    #[inline]
    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.vec.get_mut(key).map(|x| {
            x.as_mut().unwrap()
        })
    }

    pub fn iter(&self) -> FastDictIter<T> {
        FastDictIter {
            vec : (&self.vec[..]).iter()
        }
    }

    pub fn iter_mut(&mut self) -> FastDictIterMut<T> {
        FastDictIterMut {
            vec : (&mut self.vec[..]).iter_mut()
        }
    }

    pub fn traverse<F>(&mut self, n : isize, mut f: F)
        where F: FnMut(&mut T)
    {
        f(self.get_mut_no_check(n));
    }
}

pub struct FastDictIterMut<'a, T : 'a> {
    vec : IterMut<'a, Option<T>>
}

impl<'a, T> Iterator for FastDictIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        let item = self.vec.next();
        match item {
            None => None,
            Some(mut item) => match item {
                &mut None => self.next(),
                &mut Some(ref mut item) => Some(item)
            }
        }
    }
}

pub struct FastDictIter<'a, T : 'a> {
    vec : Iter<'a, Option<T>>
}

impl<'a, T> Iterator for FastDictIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.vec.next();
        match item {
            None => None,
            Some( item) => match item {
                &None => self.next(),
                &Some(ref item) => Some(item)
            }
        }
    }
}
#[test]
fn test_insert() {
    use std::collections::HashSet;

    println!("1b");
    let mut foo : FastDictionary<HashSet<i32>> = FastDictionary::new(0);
    foo.insert(0, HashSet::new());
    println!("2");
}
