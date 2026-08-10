// ===========================================================
//  RUST - STRUCTS (Chapter 1)
//  Topics Covered:
//   1. Basic Struct Definition and Instantiation
//   2. Passing Struct to Functions (Borrowing)
//   3. Methods using impl block (&self)
//   4. Multiple impl blocks (&mut self)
//   5. Updating Struct fields with a method
//   6. Struct Update Syntax (..)
//   7. Tuple Structs
//   8. Unit-like Structs
//   9. Optional Fields using Option<T>
//  10. Struct Destructuring
// ===========================================================

// #[derive(Debug)] - yeh Rust ka built-in attribute hai
// Isse struct ko println! me {:?} ya {:#?} format se print kar sakte hain
// Bina iske struct directly print nahi hota (Debug trait implement nahi hoti)
// Yeh attribute struct definition se theek pehle likhte hain (neeche dekho Topic 10)

// ===========================================================
// TOPIC 1: BASIC STRUCT DEFINITION AND INSTANTIATION
// Struct = ek custom data type jisme multiple related fields hote hain
// Java/C++ ke class jaisa, lekin Rust me methods struct ke andar nahi likhte
// Sab fields initialize karne zaroori hain (no default values by default)
// ===========================================================
/*
struct User {
    name: String,      // String type - heap pe store hota hai
    age: u32,          // u32 = unsigned 32-bit integer (0 se ~4 billion tak)
    is_active: bool    // boolean field (true ya false)
}

fn main() {
    // Struct ka ek instance banana (C++ ka object jaisa)
    let user = User {
        name: String::from("Raju"),  // String::from() heap me string banata hai
        age: 27,
        is_active: true,
    };
    // {:?} = Debug format se print karo (derive(Debug) zaroori hai)
    // {:#?} = pretty-print (indented) ke liye
    println!("{:?}", user)
}
*/
// ===========================================================
// TOPIC 2: SELF TYPES - REFERENCE GUIDE
// Teen tarike hain struct methods me self use karne ke:
// &self       -> Sirf READ karo (immutable borrow) - original safe rehta hai
// &mut self   -> MODIFY karo (mutable borrow) - ownership nahi jaati
// self        -> Ownership le lo (move/consume) - baad me use nahi hoga
// ===========================================================
/*
&self       -> read
&mut self   -> modify
self        -> ownership consume (move)
*/


// ===========================================================
// TOPIC 3: STRUCT PASS TO FUNCTION (MUTABLE BORROW - &mut)
// Function ko struct ka mutable reference dena
// Isse function struct ke fields modify kar sakta hai
// ===========================================================
// struct User {
//     name: String,
//     age: u32,
// }

// // &mut User - mutable reference pass kar rahe hain function me
// fn print_user(user: &mut User) {
//     println!("{}", user.name);   // read karo - name print karo
//     user.age = 25;               // modify karo - age badal diya
// }

// fn main() {
//     let mut user = User {        // `mut` zaroori hai, tabhi &mut pass hoga
//         name: String::from("Raju"),
//         age: 27,
//     };
//     print_user(&mut user);       // mutable reference diya function ko
//     println!("{:?}", user);      // original user ka updated data print hoga
// }

// ===========================================================
// TOPIC 4 AND 5: impl BLOCK - STRUCT KE METHODS DEFINE KARNA
// impl = implementation block
// &self method = read only access
// &mut self method = struct ke data ko modify kar sakte hain
// Multiple impl blocks - ek hi struct ke liye alag-alag impl likh sakte hain
// ===========================================================
/*
struct User {
    name: String,
    age: u32,
    is_active: bool,
}

// Pehla impl block - immutable method (&self)
impl User {
    fn print_user(&self) {
        println!("{}", self.name)  // self se current instance ka field access karo
    }
}

// Doosra impl block - mutable method (&mut self)
// Rust me ek struct ke liye multiple impl blocks allowed hain
impl User {
    fn set_age(&mut self) {
        self.age = 25   // age field modify kar diya
    }
}

fn main() {
    let mut user = User {           // `mut` zaroori hai kyunki &mut self call karenge
        name: String::from("Raju"),
        age: 27,
        is_active: false,
    };
    user.print_user();   // &self method - sirf read kiya
    user.set_age();      // &mut self method - age modify kiya
    println!("{:?}", user)
}
*/

// ===========================================================
// TOPIC 6: impl METHOD SE STRUCT UPDATE KARNA
// Method ke andar &mut self lekar dono fields update karo
// ===========================================================
/*
struct User {
    name: String,
    age: u32,
}

impl User {
    // &mut self - struct ke fields ko update karna
    fn update(&mut self, name: String, age: u32) {
        self.name = name;   // naya naam assign karo
        self.age = age      // naya age assign karo
    }
}

fn main() {
    let mut user = User {
        name: String::from("Raju"),
        age: 27,
    };
    user.update(String::from("Raj"), 25);  // method call ke saath naye values
    println!("{:?}", user)                 // "Raj", 25 print hoga
}
*/

// ===========================================================
// TOPIC 7: STRUCT UPDATE SYNTAX  ..existing_struct
// Kuch fields naye do, baaki fields purane struct se copy ho jaayenge
// Note: Agar String field copy ho toh ownership move ho jaati hai
// ===========================================================
// struct User {
//     name: String,
//     age: u32,
//     is_active: bool,
// }
//
// fn main() {
//     let user1 = User {
//         name: String::from("Raju"),
//         age: 25,
//         is_active: false,
//     };
//     // ..user1 = baaki fields (age, is_active) user1 se copy ho gayi
//     let user2 = User {
//         name: String::from("Kumar"),
//         ..user1
//     };
//     println!("{:?}", user2);
// }


// ===========================================================
// TOPIC 8: TUPLE STRUCT
// Field names nahi hote, index se access karte hain (.0, .1)
// Use-case: Jab sirf type matter kare, field naam nahi
// ===========================================================
/*
struct Colors(String, u32);  // tuple struct - koi field name nahi

fn main() {
    let colors = Colors(String::from("black"), 0);
    println!("{}", colors.0);   // "black" - index 0
    println!("{}", colors.1);   // 0       - index 1
}
*/


// ===========================================================
// TOPIC 9: UNIT-LIKE STRUCT (Empty Struct)
// Koi field nahi hota, sirf ek type ka kaam karta hai
// Use-case: Compile-time type safety enforce karna
//           Sirf Admin delete kar sake, Guest nahi
// ===========================================================
/*
struct Admin;   // koi field nahi - sirf ek named type
struct Guest;   // alag type hai

fn delete_user(_admin: Admin) {  // sirf Admin type accept karta hai
    println!("user deleted")
}

fn main() {
    delete_user(Admin);    // Admin diya - chalega
    // delete_user(Guest);  // Guest diya - COMPILE ERROR (type mismatch)
}
*/


// ===========================================================
// TOPIC 10: OPTION<T> FIELD IN STRUCT
// Option<T> = ya toh Some(value) hoga, ya None hoga
// Yeh Rust me null ki jagah use hota hai - safer hai
// Use-case: Jab koi field optional ho (hona bhi sakta hai, nahi bhi)
// ===========================================================
#[derive(Debug)]
struct User {
    name: String,
    age: Option<u32>,   // age optional hai - Some(25) ya None ho sakta hai
    is_active: bool,
}


// ===========================================================
// TOPIC 11: STRUCT DESTRUCTURING
// Struct ke fields ko seedha alag-alag variables me unpack karna
// Syntax: let StructName { field1, field2, .. } = instance;
// .. (rest pattern) = baaki ke fields ko ignore karo
// ===========================================================
fn main() {
    let user = User {
        name: String::from("Raju"),
        age: Some(25),   // Some(value) - age hai
        is_active: true,
    };

    // Destructuring: name aur age alag variables me nikal liye
    // .. = baaki (is_active) field ignore karo
    let User { name, age, .. } = user;

    print!("{}", name);    // "Raju" print hoga
    print!("{:?}", age);   // Some(25) print hoga ({:?} kyunki Option type hai)
}
