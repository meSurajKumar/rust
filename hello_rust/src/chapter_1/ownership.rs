//  Chapter 1 - Ownership
/*
Ye Rust ka sabse basic important concept hai
Rust me grabage collector nahi hota.

Ownership is a Rust memory management system where every value has exactly one owner.
The owner is responsible for the lifetime of that value, and when the owner goes out of scope,
Rust automatically frees the memory. Ownership can be moved, borrowed, or cloned depending on how the value is used.

Ya Hindi me:

Ownership ka matlab hai ki Rust me har value ka sirf ek owner hota hai. Wahi owner us memory ka responsible hota hai.
Agar ownership kisi dusre variable ya function ko move ho jaye to purana owner invalid ho jata hai.
Jab owner scope se bahar chala jata hai to Rust automatically memory free kar deta hai.


Java.
```
new Object()
GC baad me delete kr dega.
```

Node.js
```
let obj ={}
V8 cleanup karega
```

Rust
Rule 1.
```
let s = String::from("Hello");
Owner = s
```

Rule 2.
Ek time pr sirf ek owner
```
let s1 = String::from("Hello");
let s2 = s1;

Ab s2 ->Owner hai.

and S1 invalid ho gaya
and rust me esko Move kahte hai
println!("{}",s1) // ye ab Invalid hai and ab esme compiler Error mile ga.

```

Rule 3

Owner scope se bahar gaya to memory free
{
    let s = String::from("Hello");
}
Scope end so memory free , and ye automatic hota hai.

---- Clone vs Move ----
Move
```
let s1 = String::from("Hellow");
let s2 = s1;
Kyu ki Heap copy nahi hoti, Ownership transfer hoti hai.
```
Clone
```
let s1 = Strign::from("Hellow");
let s2= s1.clone();
and ab yaha pr println!(s1) and println!(s2) dono he valid hai.
Valid kyu hai kyu ki dono ki pass alag-alag heap allocation hai.
```

------ Stack Vs Heap------
Ye bhut  important hai.
let x = 10;
let y = x;
Yaha pr copy hoga , Kyun ? Integer Stack me hai
Data Type jinme Copy Trait hai
    i32
    bool
    char
    f64

PR-
let s1 = String::from("Hellow");
let s2 = s1;
Heap Allocation hoga.Kyu ?(Strign copy hoti hai) To Move hoga to s1 invalid ho gaya. 
*/

/*

Code Example Of Ownership.

*/
// Very basic exmple of Ownership
// fn main(){
//     let name = String::from("Raj");
//     println!("{}",name)
// }

/*
Stack

+-----------+
| name      |
| ptr ------|---------------------+
| len = 5   |                     |
| cap = 5   |                     |
+-----------+                     |
                                  |
Heap                              |
+----------------------+          |
| "Suraj"              |<---------+
+----------------------+

*/

//  Example 2 -- Scope end
// fn main(){
//     {
//         let city = String::from("Delhi");
//         println!("{}",city);
//     }
// }

/*
Scope start
city ---> "Delhi"
Scope end

city (removed from memory)
heap cleaned.
Heap Memory Free

Rust automatically drop call karta hai jab owner scope ke bahar chala jata hai. Isi wajah se manual free() ki zarurat nahi padti
*/

// Example 3.1 : Move in case of string (Healp allocation)
/*
fn main(){
    let s1 = String::from("Hello");
    let s2 = s1;
    
    // println!("{}",s1); // invalid because ownership transfered to the s2.
    println!("{}",s2); // Kyu ki ye string hai to move ho ga and s2 ko s1 ka heap allocation ho gaya hai, to ab s1 valid nahi rahi ga and s2 he original owner hoga.
    
}
*/

// Example 3.2 Copy in i32 like datatype and where stack is used.
// Kyu ki yaha pr stack use ho raha hai to yaha pr copy hoga , n1 and n2 dono valid hai.
/*
fn main(){
    let n1 = 3;
    let n2 = n1;
    println!("n1 :  {}",n1);
    println!("n2 :  {}",n2);
}
*/

// Funtion Ownership // Who is the owner here.
/*
fn print_name(name:String){
    println!("{}",name);
}

fn main(){
    let user = String::from("Raj");
    print_name(user);
}
*/
// Har function call ownership transfer kar sakta hai agar parameter by value liya gaya ho.
/*
fn main(){
    let s1 = String::from("Hellow");
    let s2 = s1;
    let s3 = s2;
    
    // println!("{}",s1) Both have transfer there ownership to s2
    // println!("{}",s1)Both have transfer there ownership to s3
    println!("{}",s3)
}
*/

// fn take(value:String){
//     println!("{}",value);
// }

// fn main(){
//     let name = String::from("Raju");
//     take(name);
//     take(name); // error because name is already moved to "value"
// }
/*
fn main(){
    let x = 100000;
    let y = x;
    println!("{}",x);
    println!("{}",y);
}
*/

/*

fn get_name()->String{
    let name = String::from("Raju ji");
    name
    
    }
    fn main(){
        let user = get_name();
        println!("{}",user);
        }
        /*
        Rust me return value bhi ownership transfer karti hai. Function ke end par drop tabhi hota hai jab ownership kisi aur ko move na hui ho
        */
*/
/*
fn get_name() -> String{
    let name = String::from("OK");
    println!("get_name : {}",name);
    name
}

fn main(){
    let user = get_name();
    println!("main : {}",user);
}

*/

fn main(){
    let s1 = String::from("Raju");
    let s2 = s1;
    let s3 = s2;
    println!("{}",s3)
}