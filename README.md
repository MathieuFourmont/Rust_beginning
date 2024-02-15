# Intro

fn main() {} -> declares a function that returns nothing

println! = keyword for printing -> it is a macro that takes "Hello world!", the macro is defined by the '!' before the parenthesis:  
`println!("Hello World!\n");`

Macros are used in metaprogramming, i.e., code that writes code.  
They look like functions in other system programming languages like C and C++, but instead of generating a function call like functions, they are expanded into source code that gets compiled with the rest of the program.
        
### Positional arguments:
`println!("Enhance your coding skills from {0} courses. {0} courses are very {1}\n", "Educative", "interactive");`
    
### Named arguments:  
`println!("{company} provides {kind} courses\n\n", company = "Educative", kind = "interactive");`
    
### Placeholder traits:  
`println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}\n", 10, 10, 10);`
    
### Basic Math:  
`println!("{} + {} = {}\n",10, 10, 10 + 10);`
    
### Placeholder for a Debug Trait:  
- It is possible to display multiple values using a single placeholder with the help of the debug trait (a colon followed by a question mark {:?}).
- You can use a debug trait and write as many values as desired within the parentheses:  
`println!("{:?}\n", ("This is a Rust Course", 101));`

# Variables

Variable creation
- To declare and assignes a new mutable variable x = 10, 'mut' keyword is needed if we want to reassign the value of x later.
- Rust compiler can infer from the type of value assigned to it, but if we want to declare explicitly a variable, we need to specify the data typeafter the name of the variable:  
`let mut x:i32 = 10;` ':i32' (int 32-bits signed) is the data type  
`println!("Mutable varibable x = {}", x);`  
`x = 20;`  
`println!("New value of x = {}\n", x);`

# Data types

## Numeric Types

### Integers:

#### Fixed Size Types

The fixed integer types have a specific number of bits in their notation.

- i8: The 8-bit signed integer type.
- i16: The 16-bit signed integer type.
- i32: The 32-bit signed integer type.
- i64: The 64-bit signed integer type.
- u8: The 8-bit unsigned integer type.
- u16: The 16-bit unsigned integer type.
- u32: The 32-bit unsigned integer type.
- u64: The 64-bit unsigned integer type.

#### Variable Size Types

The integer type in which the particular size depends on the underlying machine architecture

- isize: The pointer-sized signed integer type.
- usize: The pointer-sized unsigned integer type.

Integers can be declared explicitly:  
`let a:i32 = 24;`  
`let b:u64 = 23;`  
`let c:usize = 26;`  
`let d:isize = 29;`  
or implicitly:  
`let a = 343434`

### Floating-point numbers:

Refer to numbers with a fractional part.  
The representation of floating-point numbers in a computer’s memory is such that the precision with which a number is stored in memory depends on the number of bits used for storing the variable.

There are 2 subtypes:
- f32: The 32-bit floating point type.
- f64: The 64-bit floating point type.

Explicit declaration:  
`let f1:f32 = 32.9;`  
`let f2:f64 = 6789.89;`  
Implicit:  
`let pi = 3.14;`  
`let e = 2.17828;`

## Boolean

- Explicit declaration:  
`let is_bool:bool = true;`  
- Implicit:  
`let a = true;`

Result of an Expression:  
The result of an expression that evaluates to either true or false (for example a comparison of two values) can be assigned to an implicit boolean variable.  
`let c = 10 > 2;`  
`println!("c: {}", c);`  
Output:  
`c: true`

## Character and String

### Char

- Explicit declaration:  
`let char_1:char = 'e';`  
- Implicit:  
`let char_2 = 'a';`

### String

- Explicit declaration:  
`let str_1:&str = "Rust Programming";`  
- Implicit:  
`let str_2 = "Rust Programming";`

## Arrays

 It is used when the collection of values of the same type are to be stored in a single variable. In Rust, an array can only be of a fixed length.

 ### Define an Array

 To define an array in Rust, we have to define the type and size of the array.

- Explicit declaration:  
 `let arr:[i32;4] = [1, 2, 3, 4];` -> declares an array of size 4 with elements 1, 2, 3, 4.
- Implicit:  
`let arr1 = [0 ; 4];` -> initialize an array of size 4 with 0
- Mutable:  
`let mut arr:[i32;4] = [1, 2, 3, 4];`  
`arr[1] = 9;`

### Access an Element of an Array

`let arr:[i32;4] = [1, 2, 3, 4];`  
`println!("The first value of array is {}", arr[0]);`

### Print the Array

The whole array can be traversed using a loop or the debug trait.

`let arr:[i32;4] = [1, 2, 3, 4];`  
`println!("\nPrint using a debug trait");`  
`println!("Array: {:?}", arr);`

### Get the Length of the Array

`let arr:[i32;4] = [1, 2, 3, 4];`  
`println!("Length of array: {}", arr.len());`

### Get Slice

Slice is basically a portion of an array. It lets you refer to a subset of a contiguous memory location. But unlike an array, the size of the slice is not known at compile time.

#### Syntax

A slice is a two-word object, the first word is a data pointer and the second word is a slice length.  
To declare an array slice, we need to specify the name of the source array and the range of elements to be included in the slice.

`let arr:[i32;4] = [1, 2, 3, 4];`  
`let slice_array1:&[i32] = &arr;` -> define the slice  
`let slice_array2:&[i32] = &arr[0..2];`  
`println!("Slice of an array: {:?}", slice_array1);`  
`println!("Slice of an array: {:?}", slice_array2);`  
Output:  
"Slice of an array: [1, 2, 3, 4]"  
"Slice of an array: [1, 2]"

## Tuples

Tuples are heterogeneous sequences of elements, meaning, each element in a tuple can have a different data type. Just like arrays, tuples are of a fixed length.

### Define a Tuple

A tuple can be defined by writing let followed by the name of the tuple and then enclosing the values within the parenthesis.

- Without specifying the type:  
`let person_data = ("Alex", 48, "35kg", "6ft");`
- With it:  
`let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");`

### Access the Value of the Tuple

2 diffrent ways:
- Using the '.' operator:  
`let person_data = ("Alex", 48, "35kg", "6ft");`  
`println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);`
- OR we can use pattern matching to destructure a tuple value:  
`let person_data = ("Alex", 48, "35kg", "6ft");`  
`let (w, x, y, z) = person_data;`  
`println!("Name : {}", w);`

## Constant Variables

Constant variables are ones that are declared constant throughout the program scope, meaning, their value cannot be modified. They can be defined in global and local scope.

Naming Convention:  
By convention, you write a constant variable name in a SCREAMING_SNAKE_CASE, i.e.,
- All letters should be UPPER case.
- All words should be separated using an underscore ( _ ).

`const ID_1: i32 = 4;`

### Difference Between const and let Variables

#### Declaration
Constant variables are declared using the const keyword unlike let variables.

#### Scope
const variables are declared in global and local scope unlike let variables that are declared only in the local scope.

#### Mutability
const variable cannot be mutable unlike let which can be made mutable using mut keyword.

#### Data Type
Unlike let variables, it is mandatory to define the data type of const variables.

#### Set Value at Run-time
The value of const variable can only be set before running the program whereas the let variable can store the result at runtime.

#### Shadowing
Unlike let variables, const variables cannot be shadowed.

# Operators

## Intro

An operator is a symbol that takes one or more values and outputs another. It tells the compiler to perform some sort of operation.

Different operators are available in Rust for performing different operations. Based on the number of operands, the operators can be categorized into binary and unary operators:

### Unary Operators

The operators that act upon a single operand are unary operators.

- Borrow expression (&, &mut)
- Dereference expression (*)
- Negation expression (!)

### Binary Operators

The operators that deal with two operands are binary operators.

- Arithmetic expression (+, -, *, /, %)
- Logical expression (&&, ||)
- Comparison expression (>, <, <=, >=, ==, !=)
- Bitwise expressions (&, |, ^)
- Assignment expression (=)
- Compound Assignment expression (-=, +=, /=, %=, *=)
- Typecast expression (as)

## Type Casting Operator

Type casting is when you convert the data type of the variable to some other data type.  
In Rust, typecasting is done using the `as` keyword followed by the desired data type of the variable or value.

`let a = 15;`  
`let b = (a as f64) / 2.0;`   
`println!("a: {}", a);`  
`println!("b: {}", b);`  
Output:  
`a: 15`  
`b: 7.5`

What data types can be type casted?  
- integer to floating-point and vice versa
- integer to String

But:
- String (&str) or character cannot be type casted to the data type of type integer or float
- Character cannot be type casted to String type and vice versa

## Borrowing and Dereferencing Operators

### Borrowing Operator

Borrowing means to reference the original data binding or to share the data. (References are just like pointers in C.)

Two variables are involved in a borrowing relationship when the referenced variable holds a value that the referencing variable borrows. The referencing variable simply points to the memory location of the referenced variable.

There are 2 types:
- Shared borrowing:  
A piece of data that is shared by single or multiple variables but it cannot be altered
- Mutable borrowing:  
A piece of data that is shared and altered by a single variable (but the data is inaccessible to other variables at that time)

Mutable references (mutable borrow operations) are moved while immutable references (shared borrow operations) are copied.

Ex:  
`fn main() {`  
`let x = 10;`  
`let mut y = 13;`  
//immutable reference to a variable  
`let a = &x;`  
`println!("Value of a:{}", a);`  
`println!("Value of x:{}", x);` // x value remains the same since it is immutably borrowed

//mutable reference to a variable  
`let b = &mut y;`  
`println!("Value of b:{}", b);`  
`*b = 11;` // derefencing  
`println!("Value of b:{}", b);` // updated value of b  
`println!("Value of y:{}", y);` // y value can be changed as it is mutuably borrowed  
`}`

Output:  
`Value of a:10`  
`Value of x:10`  
`Value of b:13`  
`Value of b:11`  
`Value of y:11`

### Dereferencing Operator

Once you have a mutable reference to a variable, dereferencing is the term used to refer to changing the value of the referenced variable using its address stored in the referring variable. (`*a = b`)

Ex:  
`fn main() {`  
//mutable reference to a variable  
`let mut x = 10;`  
`println!("Value of x:{}", x);`  
`let a = & mut x;`  
`println!("Value of a:{}", a);`  
//dereference a variable  
`*a = 11;`  
`println!("Value of a:{}", a);`  
`println!("Value of x:{}", x);` // Note that value of x is updated  
`}`  
Output:  
`Value of x:10`  
`Value of a:10`  
`Value of a:11`  
`Value of x:11`

## Precedence and Associativity

### Precedence

The precedence of an operator determines which operation is performed first in an expression with more than one operators.

Operators are listed below in the order of their precedence from highest to lowest:  
- Unary:
	- Logical/Bitwise NOT: `!`
	- Dereference: `*`
	- Borrow: `&`, `&mut`
- Binary:
	- Typecast: `as`
	- Multiplication: `*`, Division: `/`, Remainder: `%`
	- Addition: `+`, Subtraction: `-`
	- Left Shift: `<<`, Right Shift: `>>`
	- Bitwise AND: `&`
	- Bitwise XOR: `^`
	- Bitwise OR: `|`
	- Comparison: `==` `!=` `<` `>` `<=` `>=`
	- Logical AND: `&&`
	- Logical OR: `||`
	- Range: `start .. stop`
	- Assignment/Compound Assignment: `=` `+=` `-=` `*=` `/=` `%=` `&=` `|=` `^=` `<<=` `>>=`

### Associativity

If two or more operators of the same precedence appear in a statement, then which operator will be evaluated first is defined by the associativity.

#### Left to Right Associativity

Left associativity occurs when an expression is evaluated from left to right. An expression such as `a ~ b ~ c`, in this case, would be interpreted as `(a ~ b) ~ c` where `~` can be any operator.

The operators below can be chained as left associative:  
- `as`
- `*`, `/`, `%`
- `+`, `-`
- `<<`, `>>`
- `&`
- `^`
- `|`
- `&&`
- `||`

Ex:  
1.  `println!("Answer : {}",( 3 + 5 ) * 9 / 7 & 8);`
- (3 + 5) = 8
- 8 * 9 = 72
- 72 / 7 = 10
- 10 & 8 = 8

2. `println!("{}", 2 + 3 / 5 ^ 7 & 8 | 9);`
- 3 / 5 = 0
	- 2 + 0 ^ 7 & 8 | 9
- 2 + 0 = 2
	- 2 ^ 7 & 8 | 9
- 7 & 8 = 0
	- 2 ^ 0 | 9
- 2 ^ 0 = 2
	- 2 | 9 = 11

## Challenge

How to calculate (a + b) ^ 3

`fn main() {`  
`let a = 2;`  
`let b = 2;`  

`let c = i32::pow(a,3) + i32::pow(b,3) + ( 3 * a * b * (a + b));`  
`println!("{}", c);`  
`}`