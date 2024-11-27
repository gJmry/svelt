pub fn main(name: String, typescript: bool) {
    match typescript {
        true => { println!("Type script"); }
        false => { println!("Javascript"); }
    }
}