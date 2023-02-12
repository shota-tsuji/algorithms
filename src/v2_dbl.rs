use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

// To avoid reference cycles, `prev` is defined as `Weak<T>` whilst `next` uses strong reference
// with `Rc<T>`.
// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html
#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                // set a weak reference.
                self.last = Some(Rc::downgrade(&new_back));
                // set a strong reference.
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                // set a weak reference.
                self.last = Some(Rc::downgrade(&new_data));
                // set a strong reference.
                self.first = Some(new_data);
            }
        }
    }
}

fn main() {
    let mut dl = DbList::new();
    dl.push_front(6);
    dl.push_back(11);
    dl.push_front(5);
    dl.push_front(4);
    dl.push_front(3);
    dl.push_back(15);

    println!("dl {:?}", dl);
}
