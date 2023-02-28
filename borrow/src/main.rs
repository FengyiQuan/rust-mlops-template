
// Modules
mod borrowing;
mod mutability;
mod aliasing;
mod ref_pattern;

// main
fn main(){
    borrowing::test();
    mutability::test();
    aliasing::test();
    ref_pattern::test();
}