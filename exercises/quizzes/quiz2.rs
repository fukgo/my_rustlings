// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function.
    // pub fn transformer(input: ???) -> ??? { ??? }
    //input : a Vector of 2-length tuples
    pub fn transformer(input:Vec<(String,Command)>)->Vec<String>{
        let mut output:Vec<String> = Vec::new();
        for (input_str,command) in input{
            match command{
                Command::Uppercase=>{output.push(input_str.to_uppercase().to_string())},
                Command::Append(num)=>{output.push(format!("{}{}",input_str,"bar".repeat(num.into())))},
                Command::Trim=>{output.push(input_str.trim().to_string())}
            }
            
        }
    output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            vec![
                "HELLO".to_string(),
                "all roads lead to rome!".to_string(),
                "foobar".to_string(),
                "barbarbarbarbarbar".to_string(),
            ]
        );
    }
}
