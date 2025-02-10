fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x;    // z is an immutable reference to x
    *y += 1;       // Modifies x through y
    println!("x = {}", x); // prints x = 6
    println!("z = {}", *z); // prints z = 6;  this is okay

    // The following line is not okay because of dangling pointer
    let w = &*y;   // w is another reference to x
    // w is a dangling pointer because the lifetime of the value pointed to by w ends in this line
    *y = 10;       // This line may modify the memory address of w
    println!("w = {}", *w); // Danger: Undefined behavior because w is a dangling pointer
}