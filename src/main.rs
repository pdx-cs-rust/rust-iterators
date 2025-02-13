struct Counter {
    start: usize,
    end: usize,
}

impl Counter {
    fn new(r: std::ops::Range<usize>) -> Self {
        Counter {
            start: r.start,
            end: r.end,
        }
    }

}

fn range_iter(mut r: std::ops::Range<usize>) -> impl Iterator<Item = usize> {
    std::iter::from_fn(move || {
        if r.start < r.end {
            let result = r.start;
            r.start += 1;
            Some(result)
        } else {
            None
        }
    })
}

fn range_iter_generic<T>(mut r: std::ops::Range<T>) -> impl Iterator<Item = T>
    where
       T: std::cmp::PartialOrd + std::ops::AddAssign + Clone + num_traits::Num,
{
    std::iter::from_fn(move || {
        if r.start < r.end {
            let result = r.start.clone();
            r.start += num_traits::one();
            Some(result)
        } else {
            None
        }
    })
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let Counter{ ref mut start, ref end } = self;
        if *start < *end {
            let result = *start;
            *start += 1;
            Some(result)
        } else {
            None
        }
    }
}

/*

struct Skipper<T> {
    iter: T,
    skip: Option<usize>,
}


impl<T> Skipper<T> {
    fn new(iter: T) -> Self {
        Skipper {
            iter,
            skip: Some(0),
        }
    }
}

impl<T: Iterator<Item=usize>> Iterator for Skipper<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(skip) = self.skip {
            let result = self.iter.skip(skip).next();
            self.skip = result;
            result
        } else {
            None
        }
    }
}

/*
impl<T: Iterator<Item=usize>> IntoIterator for Skipper<T> {
    type Item = usize;
    type IntoIter = Skipper<T>;

    fn into_iter(self) -> Self::IntoIter {
        Skipper::new(self)
    }
}
*/
*/

fn main() {
    for v in Counter::new(0..2) {
        println!("{}", v);
    }
    println!();

    for v in 0..3 {
        println!("{}", v);
    }
    println!();

    for v in range_iter(0..4) {
        println!("{}", v);
    }
    println!();

    for v in range_iter_generic(0u16..5) {
        println!("{}", v);
    }
    println!();

    /*
    fn next(&mut self) -> Option<Self::Item> {
        ...
        *self = &self[1..]
        ...
    }
    */

    for &v in &['a', 'b', 'c'] {
        println!("{}", v);
    }
    println!();

    for v in &mut ['w', 'x', 'y'] {
        *v = char::from(*v as u8 + 1);
        println!("{}", v);
    }
    println!();

    /*
    for x in c {
        ...
    }

    // c is a value type
    let _c = c.into_iter();
    // c is a reference type
    let _c = c.iter();
    // c is a mutable reference type
    let _c = c.iter_mut();
    
    // x has the type that matches _c
    while let Some(x) = _c.next() {
        ...
    }
    */

    let s: String = ["s", "t", "u"].into_iter().collect();
    println!("{}", s);
    println!();

    let c = 0..2;
    for v in c {
        println!("{}", v);
    }
    println!();

    let mut x = (0..17)
        .map(|v| {
            println!("{}", v);
            v + 1
        });
    let _ = x.next();
}
