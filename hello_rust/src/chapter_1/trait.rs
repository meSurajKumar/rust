// ============================================================
// TRAITS IN RUST
// ============================================================
// Trait ek tarah ka "contract" hota hai jo define karta hai
// ki koi bhi type kya-kya kaam kar sakta hai (behavior/methods).
// Jaise Java mein Interface hota hai, Rust mein Trait hota hai.
//
// Syntax:
//   trait TraitName {
//       fn method_name(&self);  // sirf signature, koi body nahi
//   }
// ============================================================

// #[derive(Debug)] — Rust compiler ko bolta hai ki is struct ke liye
// automatically Debug trait implement kar do, taaki {:?} se print ho sake.
#[derive(Debug)]

// ----------------------------------------------------------
// STRUCTS — Data store karne ke liye custom types
// ----------------------------------------------------------

// User struct — ek regular user ko represent karta hai
struct User {
    name: String, // user ka naam
}

// Admin struct — ek admin user ko represent karta hai
struct Admin {
    name: String, // admin ka naam
}

// ----------------------------------------------------------
// TRAIT DEFINITION
// ----------------------------------------------------------
// PrintInfo ek shared behavior define karta hai.
// Jo bhi type is trait ko implement karega, usse print_fn() banana padega.
trait PrintInfo {
    fn print_fn(&self); // &self matlab: apne aap ka reference lega (ownership nahi lega)
}

// ----------------------------------------------------------
// TRAIT IMPLEMENTATION — User ke liye
// ----------------------------------------------------------
// "impl TraitName for Type" se hum kisi type ke liye trait implement karte hain.
impl PrintInfo for User {
    fn print_fn(&self) {
        // self.name — User struct ka name field access kar raha hai
        println!("{}", self.name);
    }
}

// ----------------------------------------------------------
// TRAIT IMPLEMENTATION — Admin ke liye
// ----------------------------------------------------------
impl PrintInfo for Admin {
    fn print_fn(&self) {
        // Admin ka naam print kar raha hai
        println!("{}", self.name);
    }
}

// ----------------------------------------------------------
// GENERIC FUNCTION using `impl Trait` syntax
// ----------------------------------------------------------
// Yeh function kisi bhi aisi type ka reference accept karta hai
// jo PrintInfo trait implement karta ho.
// Iska faida: ek hi function User aur Admin dono ke saath kaam karta hai!
//
// &impl PrintInfo  =>  "kuch bhi do, bas PrintInfo implement karo"
fn print_data(info: &impl PrintInfo) {
    info.print_fn(); // trait method call kar raha hai
}

// ----------------------------------------------------------
// MAIN FUNCTION — Entry Point
// ----------------------------------------------------------
fn main() {
    // User ka instance banana
    let user = User {
        name: String::from("User Raju"),
    };

    // Admin ka instance banana
    let admin = Admin {
        name: String::from("Admin Hitest"),
    };

    // Direct trait method call bhi kar sakte hain (abhi commented out hai):
    // user.print_fn();
    // admin.print_fn();

    // Generic function ke through call — ek hi function dono handle karta hai
    print_data(&user);  // User ka naam print hoga
    print_data(&admin); // Admin ka naam print hoga
}

/*
 * ============================================================
 * VISUAL DIAGRAM — Trait ka flow samajhne ke liye
 * ============================================================
 *
 *              PrintInfo  (Trait / Contract)
 *                  │
 *            print_fn()   (method jo implement karna hai)
 *                  │
 *        ┌─────────┴─────────┐
 *        │                   │
 *      User               Admin
 *        │                   │
 *   implements          implements
 *        │                   │
 *        └─────────┬─────────┘
 *                  │
 *                  ↓
 *           print_data()    (generic function)
 *                  │
 *           &impl PrintInfo  (koi bhi PrintInfo wala)
 *                  │
 *           ┌──────┴──────┐
 *           ↓             ↓
 *         &User         &Admin
 *           │             │
 *           ↓             ↓
 *       print_fn()    print_fn()   (respective implementation chalti hai)
 *
 * ============================================================
 */
