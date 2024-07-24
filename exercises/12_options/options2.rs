fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(&word, &target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        //pop()方法返回的是一个 Option 类型，这个 Option 类型包含了你的 Option<i8>
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(&word, &target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        //pop()方法返回的是一个 Option 类型，这个 Option 类型包含了你的 Option<i8>
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}


    }
}
