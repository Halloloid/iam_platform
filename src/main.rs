use iam_platform::config::db_config::connect_db;



#[tokio::main]
async fn main(){

    if let Err(e) = connect_db().await{
        println!("There is a Error:{}",e);
    }
    
}
