fn main() {
    let text = "Hello. how are you?";
    let transformed_text = make_exciting(text);
    println!("Before: {text}, after: {transformed_text}");

}
fn make_exciting(s: &str) -> String {
    let s2 = s.replace(".", "!");
    let s3 = s2.replace("?", "‽");
    s3
}
