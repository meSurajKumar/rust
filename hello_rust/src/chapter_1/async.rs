use tokio::time::{sleep, Duration};

async fn fetch_user()-> String{
    sleep(Duration::from_secs(3)).await;
    "Suraj".to_string()
}

#[tokio::main]
async fn main(){
    println!("Start");
    let user = fetch_user().await;
    println!("User :{}",user);
    println!("End")
}



// https://jsonplaceholder.typicode.com/comments

