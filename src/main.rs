use std::{
    pin::Pin,
    ptr::NonNull,
    slice::{from_raw_parts, from_raw_parts_mut},
};

struct SRef<T> {
    src: Pin<Box<[T]>>,
    raw_slice: (NonNull<T>, usize),
}

impl<T: Default> SRef<T> {
    fn new() -> Self {
        let src = vec![
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        ];
        let src = src.into_boxed_slice();
        let src = unsafe { Pin::new_unchecked(src) };

        let mut res = SRef {
            src,
            raw_slice: (NonNull::dangling(), Default::default()),
        };

        //data to construct either a &[T] or &mut [T]
        let data = unsafe { res.src.as_mut().get_unchecked_mut().as_mut_ptr() };
        let len = res.src.as_ref().len();

        res.raw_slice.0 = NonNull::new(data).unwrap();
        res.raw_slice.1 = len;
        res
    }

    fn get_source_slice(&self) -> Pin<&[T]> {
        self.src.as_ref()
    }

    fn as_slice(&self) -> &[T] {
        return unsafe { from_raw_parts(self.raw_slice.0.as_ptr(), self.raw_slice.1) };
    }

    fn as_slice_mut(&mut self) -> &mut [T] {
        return unsafe { from_raw_parts_mut(self.raw_slice.0.as_ptr(), self.raw_slice.1) };
    }
}
fn main() {
    let mut x = SRef::<u8>::new();

    let a = x.get_source_slice();
    let b = x.as_slice();
    let c = x.as_slice();

    assert_eq!(b, c);

    println!("{:?}\n{:?}\n{:?}\n", a, b, c);

    //-------------mutate-------------//
    let d = x.as_slice_mut();
    d.iter_mut().for_each(|x| *x = u8::MAX);
    println!("{:?}\n", d);
    //-------------mutate-------------//

    let a = x.get_source_slice();
    let b = x.as_slice();
    let c = x.as_slice();

    assert_eq!(b, c);

    println!("{:?}\n{:?}\n{:?}\n", a, b, c);
}
