/*
Calling Rust Functions from Other Languages

We can also use extern to create an interface that allows other languages to call
Rust functions. Instead of an extern block, we add the extern keyword and specify
the ABI to use just before the fn keyword. We also need to add a #[no_mangle]
annotation to tell the Rust compiler not to mangle the name of this function.
Mangling is when a compiler changes the name we've given a function to a different
name that contains more information for other parts of the compilation process to
consume but is less human readable. Every programming language compiler mangles
names slightly differently, so for a Rust function to be nameable by other
languages, we must disable the Rust compiler's name mangling.

In the following example, we make the call_from_c function accessible from C code,
after it's compiled to a shared library and linked from C:
*/
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
