use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
        let mut y = &x;
        let z = &x;
        y.borrow_mut().push(42);
        //y.push(42);
        z.borrow_mut().push(13);
        //z.push(13);
        assert_eq!(*x.borrow(), vec![42, 13]);
    }
}
