// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {

}
    

fn string_slice(arg: &str) {
    println!("{arg}");

}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);
    //.trim() 删除字符串两端的空白字符。这些空白字符包括空格、制表符、换行符等
    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
