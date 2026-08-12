// #[derive(Debug)] // Debug trait derive karne ke liye comment kiya gaya hai (agar data print karna ho)

// Pattern Matching :- Kisi value ka structure/shape match karo and jiss pattern se match ho jaye uss code ko chala do.
/*
fn main(){ // Program ka starting point
    let number = 100; // 'number' naam ka ek variable banaya aur usme 100 store kiya
    match number{ // 'number' ki value ko alag-alag cases (patterns) ke saath match kar rahe hain
        10 => println!("Number is 10"), // Agar number 10 hai, toh ye line chalegi
        // _ => println!("Number didn't match") // '_' ka matlab 'baaki kuch bhi'. Agar koi aur value aayi toh ye chalega (abhi commented hai)
        d => println!("Number didn't match {}", d) // '_' ki jagah 'd' variable use kiya, jo match hui value ko apne andar store kar lega aur print karega
    }; // match block ka end
    
}
*/


/*
---->>> Match ko if/else jaise samajh sakte ho?
ans : partially hum aisa kar sakte hain but kyuki if/else sirf value ko match karta hai, aur jo pattern matching hai
wo sirf value ko hi nahi balki uske shape/structure (jaise Option, Tuple, Struct) ko bhi match karta hai.
*/

fn main(){ // Main function yahan se shuru hota hai
    // Niche kuch purane examples commented hain, unhe samajhne ke liye un-comment karke dekh sakte ho:
    
    // let age: Option<i32> = Some(25); // Option type ka variable, jisme value 25 hai
    // let age: Option<i32> = None; // Option type ka variable, jisme koi value nahi hai (None)
    // // println!("{:?}", age); // age ko debug format mein print karne ka tarika
    // match age{ // age variable ko match kar rahe hain
    //     Some(value) => println!("Age is {}", value), // Agar age mein Some value hai, toh us value ko nikalo (unwrap) aur print karo
    //     None => println!("No Value") // Agar age None hai, toh ye print karo
    // }  

    // let point = (0,10); // Tuple data type. Ek x aur y coordinate ka point banaya (x=0, y=10)
    // match point { // point (tuple) ke upar pattern matching lagayi
    //     (0,0) => println!("Origin"), // Agar x=0 aur y=0 hai, toh Origin print hoga
    //     (0,y) => println!("Y axis : {}", y), // Agar x=0 hai aur y kuch bhi ho, toh ye chalega aur y ki value y me aa jayegi
    //     (x,0) => println!("x axis : {}", x), // Agar y=0 hai aur x kuch bhi ho, toh ye chalega aur x ki value x me aa jayegi
    //     (x,y) => println!("some where in the points x: {} and y : {}", x, y), // Agar x aur y dono non-zero hain, toh ye dono ko catch karega
    // };


    // Niche diye gaye code me hum Option enum ka use karke pattern matching samajh rahe hain:
    
    // let age :Option<i32> = Some(18); // Ek option banaya jisme 18 store hai (isko chala kar dekh sakte ho)
    let age :Option<i32> = None; // Abhi ke liye ek Option banaya jisme koi value nahi hai (None)
    
// Option enum (jo rust me in-built hota hai) do tarah ke hote hain:
//     Some(value) -> Yani isme koi value hai
//     None -> Yani isme kuch nahi hai (Null jaisa)

    match age { // age variable par match expression lagaya
        Some(value) => println!("Value is {}", value), // Agar age mein kuch value hai, toh usko 'value' variable me daal do aur print karo
        Nones => println!("No value is given!"), // Agar age None hai, toh simply print karo ki value nahi di gayi
      
    }

} // Main function ka end



struct User {
    name : String,
    age : Option<i32>,
    is_active : bool
}

fn main (){
    let user = User{
        name : String::from("Raju"),
        age : Some(26),
        is_active : true
    };

    match user {
        User {name, age, is_active}=>{
            println!("Name : {}", name);
            println!("Age : {:?}", age);
            println!("is_active : {}", is_active)
        }
        
    }
}



