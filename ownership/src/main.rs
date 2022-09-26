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


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

}                      // this scope is now over, and s is no longer valid

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Rust would throw a compile-time error if we tried to use s after the call to takes_ownership