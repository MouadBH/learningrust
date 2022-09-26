fn main() {            // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s

    let s1 = String::from("hello");
    let s2 = s1;       // s1 was moved into s2, its not shallow copy because rust invalidate s1
                       // the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. 
    println!("{}, world!", s1); // this is cause an error cuz s1 is invalid


    let s1 = String::from("hello");
    let s2 = s1.clone(); // the heap data does get copied. its a deep copy

    println!("s1 = {}, s2 = {}", s1, s2);


    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // x is still valid and wasn’t moved into y
    // If a type implements the Copy trait, variables that use it do not move

}                      // this scope is now over, and s is no longer valid
