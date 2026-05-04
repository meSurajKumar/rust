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
fn move_example() -> String{
    let s1 = String::from("Hello");
    let s2  = s1;
    s2
    // println!("{}",s1)
}