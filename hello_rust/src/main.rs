fn main() {
    // println!("Hello, world!");
    // let mut x = 5;
    // x = x + 1;
    
    //  shadowing.
    // let x = 5;
    // let x = "userName";
    // println!("value of x is : {}" , x)

    // this is just a traditional controll statement.
    // let number = 7;
    // if number < 5{
    //     println!("number is less the 5")
    // }else if number > 5 {
    //     println!("number is grater then 5")
    // }else {
    //     println!("you are lucky")
    // } 


    // this is just in the expression form
    // let condition = false;
    // // let value = if condition {println!("your condtion is true")} else {println!("your condtion is false")};
    // let value = if condition {true} else {false};
    // println!("value is {}", value)

    // loop example 
    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("count is {}", count);
    //     if count == 10 {
    //         break;
    //     }
    // };

    // loop returning a value.    
    // let mut tick = 0;
    // let result = loop {
    //     println!("{}",tick);
    //     tick += 1;
    //     if tick == 42{
    //         break tick;
    //     };
    // };
    // println!("the value is {}", result)


    // let mut count = 0;
    // loop {
    //     count += 1;
    //     println!("count : {} ", count);
    //     if count == 5{
    //         break;
    //     }
    // } 


    // let mut tik = 0;
    // let result = loop{
    //     tik +=1;
    //     println!("tik  : {}", tik);
    //     if tik == 7{
    //         break tik;
    //     };
    // };
    // println!("this is result value : {}", result);

    // while loop
    // let mut n = 0;
    // while n <5{
    //     println!("value of n : {}", n);
    //     n +=1;
    // }

    // for loop.
    // for i in 0..5{
    //     println!("i : {}", i)
    // }
    

    // match example
    let status = 500;
    match status {
        200 => println!("OK"),
        404 => println!("Not found!"),
            _=>println!("Unkonwn status"),
    }












}
