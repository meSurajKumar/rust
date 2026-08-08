/*
Ownership ka purpose :- Ek value ka ek hi owner hoga.

AB eski problem ki baat krte hai.

Agar har har function ownership lelega to code kitan behakr ho jayeega. (Hard to maintain)

*/
/*
fn calculate_length(name:String)->(String, usize){
    let len = name.len();
    (name, len)
}

fn main(){
    let s = String::from("Raj");
    let (user, len) = calculate_length(s);
    println!("{}",user);
    println!("{}",len);
}
*/


/*
Function ko sirf strign ko read krna tha , ownerhip kyu le ?
is problem ko solve krta hai Borrowing, Borrowing ka matlab hai data ko use karo,
lekin owner mat bano, Ye reference (&T) ke through hota hai.

Uper ki exmaple me code work kr gaya, pr borrowing kyu use kran pada ya eski need kyu pade.

Borrowing is leye invent nahi huaa ki kam ho sake balkki es leye huaa taki code efficient, simple aur reusable ho. 

upar vale example me ownership edir-udhar ghum rahi bs lenght nikale ki leye.

let ast = parse(code);
lint(ast)
highlight(ast)
autocomplete(ast)
.. yaha pr har function jo hai vo ownership le raha hai. to agar hightlight(ast) agar usko ast ko use kran hai to 
lint() function ko ownership return krne pade ge. same for other...

// With ownership
let ast = parse(code);
let ast = lint(ast);
let ast = autocomplete(ast);
let ast = rename(ast);
to ye koi efficient appraoch nahi hai and agar koi function bhool gaya return krna to Game Over.

// With Borrowing. (Very easy and efficient)
let ast = parse(code);
autocomplete(&ast)
lint(&ast)
rename(&ast)

          +------------+
          |    AST     |
          +------------+
             ^
             |
Owner ------ ast
             |
   +---------+---------+---------+
   |         |         |         |
 lint   autocomplete  rename diagnostics

sab same AST ko read kr rahe hai.
ownership ek ki jahag hai.
koi return nahi.
koi move nahi.

------- Ek Aur Example
Suppose function sirf ek normal ya basic kam kr raha hai.

fn is_empty(name : String)-> bool{
    name.is_empty()
}
to kya yaha pr es function ko owner banane ki need hai ? ans : Nahi (Koi need nahi hai)

to Better Approach :-

fn is_empty(name : &String)->bool{
    name.is_empty()
}

*/

//  Now in more depth.

// Rule of Thumb (Thinking process)
// Agar function sirf read kree to : &T (Borrow)
// Agar function modify bhi kr sake to mutable Borrow : &mut T
// Agar function ko pemanent ownership dene hai to  : T

/*
fn calculate_lenght(name: &String)->usize{
    let length = name.len();
    return length;
}

fn main(){
    let user = String::from("Raj");
    let len = calculate_lenght(&user);
    println!("{}",len)
}

*/

/*
Bina Borrowing ki ye multiple name print kran possible nahi tha efficiently.

fn print_name(name: &String){
    println!("{}",name)
}

fn main(){
    let user = String::from("Rajesh");
    print_name(&user);
    print_name(&user);
    print_name(&user);
    print_name(&user);
}
*/


/*

Yahan &mut ka matlab hai:

"Ownership mat do, lekin modify karne ki permission do."

fn add_last_name(name: &mut String) {
    name.push_str(" Kumar");
}

fn main() {
    let mut user = String::from("Suraj");

    add_last_name(&mut user);

    println!("{}", user);
}

*/
/*

fn add_world(name: &mut String) {
    return name.push_str(" World");
}

fn main() {
    let mut user = String::from("Hello");
    
    add_world(&mut user);
    
    println!("{}", user);
}
*/

// ()

// fn add_sur_name(mut name:String)-> String{
//     name.push_str(" kumar");
//     let user_name = name;
//     user_name
//     // println!("{}",name)
// }


// fn main(){
//     let  user = String::from("Raju");
//     let user_name =  add_sur_name(user);
//     println!("{:?}",user_name);
// }



fn main(){
    let mut user= String::from("Raju");
    let result = user.push_str(" Kumar");
    println!("{}",user);
    println!("{:?}", result)
}
