// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    //导出宏
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
