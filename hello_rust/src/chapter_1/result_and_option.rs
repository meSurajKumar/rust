// #[derive(Debug)] // Yeh attribute struct/enum ko debug format me print karne ki capability deta hai

// =====================================================================
// RUST KE DO IMPORTANT ENUM: Option aur Result
// =====================================================================

// OPTION: Jab hume pata nahi ki koi value hai ya nahi
// Option : Is value present hai ya nahi? (Jaise agar koi user mile ya na mile)

// RESULT: Jab koi kaam succeed ya fail ho sakta hai
// Result : Kaam succeed hua ya failed? (Jaise ek number ko zero se divide karna)


// enum Option<T> {         // Option ek built-in enum hai Rust me
//     Some(T),             // Some(T) -> Value hai! Aur wo value T type ki hai
//     None                 // None   -> Koi value nahi hai (Null jaise, but safe)
// }


// =====================================================================
// EXAMPLE 1: Option ka use karke User Struct banana
// =====================================================================

// struct User {
//     id : u32,              // User ki unique ID (unsigned integer)
//     name : String,         // User ka naam
//     email : String,        // User ki email
//     age : Option<i32>,     // Age - Option isliye kyuki age dena zaroori nahi, None bhi ho sakta hai
//     is_active : bool       // User active hai ya nahi (true/false)
// }

// fn print_age(user : &User) { // Yeh function ek User ka reference leta hai aur uski age print karta hai
//     match user.age {         // user.age ko match kar rahe hain (Option<i32> hai)
//         Some(age) => {       // Agar age diya gaya hai (Some ke andar value hai)
//             println!("User age is {} ", age); // Toh age ki value print karo
//         },
//         None => {            // Agar age nahi diya gaya (None hai)
//             println!("User age is not present"); // Toh ye message print karo
//         }
//     }
// }


// fn main(){
//     let user1 = User {
//         id : 1,
//         name : String::from("Raju"),
//         email : String::from("raju@gmail.com"),
//         age : Some(27),    // user1 ki age hai - Some(27) matlab value present hai
//         is_active : true
//     };
//     let user2 = User {
//         id : 1,
//         name : String::from("Raju"),
//         email : String::from("raju@gmail.com"),
//         age : None,        // user2 ki age nahi di - None matlab value absent hai
//         is_active : false
//     };

//     print_age(&user1);     // user1 ki age print hogi: "User age is 27"
//     print_age(&user2);     // user2 ki age print hogi: "User age is not present"
// }


// =====================================================================
// EXAMPLE 2: Result ka use karke Division function banana
// =====================================================================

// enum Result<T, E> {      // Result bhi ek built-in enum hai Rust me
//     Ok(T),               // Ok(T)  -> Kaam safal raha! T type ki value return hui
//     Err(E)               // Err(E) -> Kuch galat hua! E type ki error return hui
// }

// Err(String) -> Matlab agar error aaye toh uski description String me return karenge

// Yeh function do numbers (a aur b) leta hai aur Result return karta hai
// Result<i32, String> -> Agar succeed: i32 (divide ka result), Agar fail: String (error message)
fn devide(a : i32 , b : i32) -> Result<i32, String> {
    if b == 0 {                                      // Agar b (denominator) zero hai
        Err(String::from("Can't devide by 0"))       // Toh Error return karo - zero se divide nahi kar sakte
    } else {                                         // Warna (b zero nahi hai)
        Ok(a / b)                                    // Toh division karo aur Ok me wrap karke return karo
    }
}                                                    // Note: Semicolons nahi lagaye - last line directly return hoti hai



fn main() {
    // devide(10, 5) -> 10/5 = 2, b!=0 isliye Ok(2) return hoga
    let resumt1 = devide(10, 5);

    // devide(10, 0) -> b==0 hai isliye Err("Can't devide by 0") return hoga
    let resumt2 = devide(10, 0);

    println!("{:?}", resumt1);  // Output: Ok(2)          -> Kaam safal hua
    println!("{:?}", resumt2);  // Output: Err("Can't devide by 0") -> Kaam fail hua
}

// =====================================================================
// SUMMARY: Option vs Result
// =====================================================================
// Option -> Jab value ho ya na ho  (Some / None)
// Result -> Jab kaam succeed ya fail ho (Ok / Err)
//
// Dono ko match expression se handle karte hain - safe aur clean code!
// =====================================================================