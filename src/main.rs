// declares a the function that returns nothing
fn main() {
    // println! = keyword for printing -> it is a macro that takes "Hello world!", the macro is defined by the '!' before the parenthesis 
        println!("Hello World!\n");
    // Macros are used in metaprogramming, i.e., code that writes code. They look like functions in other system programming languages like C and C++,
    //but instead of generating a function call like functions, they are expanded into source code that gets compiled with the rest of the program.
        
    // Positional arguments
        println!("Enhance your coding skills from {0} courses.  {0} courses are very {1}\n", "Educative", "interactive");
    
    // Named arguments
        println!("{company} provides {kind} courses\n\n", company = "Educative", kind = "interactive");
    
    // Placeholder traits
        println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}\n", 10, 10, 10);
    
    // Basic Math
        println!("{} + {} = {}\n",10, 10, 10 + 10);
    
    // Placeholder for a Debug Trait
        // It is possible to display multiple values using a single placeholder
        // with the help of the debug trait (a colon followed by a question mark {:?}).
        // You can use a debug trait and write as many values as desired within the parentheses.
        println!("{:?}\n", ("This is a Rust Course", 101));

    // Variable creation
        // declares and assignes a new mutable variable x = 10, 'mut' keyword is needed if we want to reassign the value of x later
        // Rust compiler can infer from the type of value assigned to it.
        // But if we want to declare explicitly a variable, we need to specify the data typeafter the name of the variable
        let mut x: i32 = 10; // ': i32' (int 32 bits) is the data type
        println!("Nutable varibable x = {}", x);
        x = 20;
        println!("New value of x = {}\n", x);
    }