



fn main() {
    let some_str = "i am going to shout";
    let shout = some_str.shout();
    let some_string = String::from("i also shout");
    let shout2 = some_string.shout();
    println!("{} - {}", shout, shout2);
}

// we deine a trait we will use to "extend" the &str and String types
pub trait SomeExtension {
    fn shout(&self) -> String;

}


// here, take our extension and implement it on any type which can be references as a str
impl<T> SomeExtension for T
where
    T: AsRef<str>,
{
    fn shout(&self) -> String {
        let s = self.as_ref();
        format!("{}!", s.to_uppercase())
    }
}
