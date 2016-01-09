mod animals;


fn main() {
    let dog = animals::Dog::new("Buddy");
    println!("Hello {}", dog.name);
    println!("Speak: {}", dog.speak());
}
