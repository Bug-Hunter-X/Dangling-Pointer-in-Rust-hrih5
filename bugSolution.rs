fn main() {
    let mut x = 5;
    { //Creating a new scope to limit the lifetime of the reference.
        let y = &mut x; // y is a mutable reference to x
        let z = &x;    // z is an immutable reference to x
        *y += 1;       // Modifies x through y
        println!("x = {}", x); // prints x = 6
        println!("z = {}", *z); // prints z = 6;  this is okay

        //Corrected version
        let w = &*y; 
        println!("w = {}", *w); // w is still valid in this scope
    } //The reference w becomes invalid once the scope is exited.
    *y = 10; //This line will not cause an error because w is no longer valid.
    // println!("w = {}", *w); // This line will now result in a compile-time error.
} 