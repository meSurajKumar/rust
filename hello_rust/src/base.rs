fn main() {
//  greet("Raj");
// let result = square(2);
// println!("square is : {}", result)
let result = move_example();
println!("result : {}", result)
}

// // fn greet(name : &str){
// //     println!("Hello ,{}!", name)
// // }  

// fn square(x : i32)-> i32{
//     // println!("x  : {}", x)
//     x * x
// }


// Borowing concept : 
// fn move_example() -> String{
//     let s1 = String::from("Hello");
//     let s2  = s1;
//     s2
    // println!("{}",s1)
// }

// 1. Ownership (Malikana Haq)
// Rust mein har value ka ek, aur sirf ek, Owner hota hai.

// Responsibility: Owner ke paas us value ki memory ko zinda rakhne aur usko destroy karne ki absolute responsibility hoti hai.

// The Drop: Jab owner ka scope (jaise function ya if-block) khatam hota hai, toh Rust automatically ek drop function call karta hai, aur Heap memory ko turant free (deallocate) kar deta hai.

// The Move: Agar tum ek variable ka data dusre variable ko dete ho (let b = a;), toh C++ ya JavaScript ki tarah yahan dono variable valid nahi rehte. Ownership pehle se dusre mein Move ho jati hai. Pehla variable wahi mar jata hai. Use dobara access karoge toh compiler error dega.

