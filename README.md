# Rust Guessing Game

### Source of base code 

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

### Key initial concepts of development in rust

#### Concept 1 - Importing Crates

By default, Rust has a few items defined in the standard library that it brings into the scope of every program. This set is called the prelude.  
If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.

In case we need to use an `external crate`

We need to modify the Cargo.toml file to include the crate as a dependency.
For using a dependency we need to add it in the [dependencies] section header that Cargo created for you.

```
Note: You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all of your dependencies locally and open it in your browser.
```

#### Concept 2 - Type of variables and references 

There are 4 important types we need to keep in mind while using libraries,external crates and coding in general

By default variables are immutable. They can be made mutable at the time of definition using `mut` keyword.  
Also references are also by default immutable. They can also be made mutable by using `mut` keyword. 

**For instance** - An important use of mutable reference is in  

```rust
let mut guess = String::new();

io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
```

Here guess is a mutable variable. When we need to take user input from CLI, we pass `&mut guess` which is a mutable reference of the guess variable.  
So now when the `stdin` library executes, after taking the input it just updates the mutable reference with the value given in CLI by user. Since we are passing the reference. The mutable guess variable gets updated automatically.

In `.read_line(&mut guess)`
The `&` indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.

References are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

#### Concept 3 - Enums and Matching

When we have to select a value from a list of possible variants we use enumeration data types. An enumerated type is declared using the enum keyword. Following is the syntax of enum −

```rust
enum enum_name {
   variant1,
   variant2,
   variant3
}
```

The match statement can be used to compare values stored in an enum. The following example defines a function, print_size, which takes CarType enum as parameter. The function compares the parameter values with a pre-defined set of constants and displays the appropriate message.  

```rust
enum CarType {
   Hatch,
   Sedan,
   SUV
}
fn print_size(car:CarType) {
   match car {
      CarType::Hatch => {
         println!("Small sized car");
      },
      CarType::Sedan => {
         println!("medium sized car");
      },
      CarType::SUV =>{
         println!("Large sized Sports Utility car");
      }
   }
}
fn main(){
   print_size(CarType::SUV);
   print_size(CarType::Hatch);
   print_size(CarType::Sedan);
}
```

Output :-  

```
Large sized Sports Utility car
Small sized car
medium sized car
```

