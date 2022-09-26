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


    // Return Values and Scope                             
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

}                      // this scope is now over, and s is no longer valid
// Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// Rust would throw a compile-time error if we tried to use s after the call to takes_ownership


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}