// 1. Ownership (Malikana Haq)
// Rust mein har value ka ek, aur sirf ek, Owner hota hai.

// Responsibility: Owner ke paas us value ki memory ko zinda rakhne aur usko destroy karne ki absolute responsibility hoti hai.

// The Drop: Jab owner ka scope (jaise function ya if-block) khatam hota hai, toh Rust automatically ek drop function call karta hai, aur Heap memory ko turant free (deallocate) kar deta hai.

// The Move: Agar tum ek variable ka data dusre variable ko dete ho (let b = a;), toh C++ ya JavaScript ki tarah yahan dono variable valid nahi rehte. Ownership pehle se dusre mein Move ho jati hai. Pehla variable wahi mar jata hai. Use dobara access karoge toh compiler error dega.

// Rust ke Ownership ke 3 Golden Rules hote hain. Chalo in teeno rules ko ek single code snippet mein break down karte hain:

// 1. Har value ka ek, aur sirf ek, Owner hota hai.

// 2. Jab owner variable scope { } se bahar nikalta hai, toh value memory se Drop (delete) ho jati hai.

// 3. Ownership ek variable se dusre variable mein Move (transfer) ho sakti hai, jiske baad purana owner invalid ho jata hai.

// Yahan yeh rules code mein kaise dikhte hain, dekho:

fn main(){
    // Rule 1 & 2 : Cretion and Drop (scope)
    {
        // 's' yahan memory ka akela owner ban gaya
        // Stack par pointer bana, Heap par "Lapce IDE" allocate hua.
        let s = String::from("Lapce IDE");  // questions {String::from} ?
        println!("Inside scope, owner is active: {}",s);
    }
    // println!("outside scope : {}",s);
    //  Ye jo uppr print ka statement hai, jase hi "s" ka scope khatam.
    // Rust ne chucahp `drop(s)` call kiya.
    // Heap memory turant free ho gayi. garbage collector ka koi wait nahi.
    // agar tum es line ko "println!("outside scope : {}",s);" uncomment krte hoto compiler error dega : "not found in this scope"

    // Rule 3 : The Move (Transferring Ownership)
    let engineer1 = String::from("Raj"); // engineer1 is the owner.
    let engineer2 = engineer1;
    // Yahan humne Ownership transfer kar di! let engineer2 = engineer1;
    // Stack me metadata copy hua, par Heap data wahi raha. // meata data and heap data ?
    // println!("engineer1 : {}",engineer1)
    println!("engineer2 : {}",engineer2);
    // ab naya malik engineer2 hai and , sirf yahi heap memory ko access kr sakta hai.
    

   

}


