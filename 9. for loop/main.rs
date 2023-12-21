fn main() {
    let animals = vec!["Rabbit", "Dog", "Cat"];

    for (index, a) in animals.iter().enumerate(){
        println!("The animal at position {} is {}", index, a);
    }
    for a in animals.iter(){
        println!("The animal name is {}", a);
    }
}
