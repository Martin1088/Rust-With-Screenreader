mod group;
use group::test_run;
/*
pub fn test() {
    let now = Utc::now();
    let unix_time = now.timestamp();
    let time_test = Utc::today();
    print!("Current time: {}", unix_time);
    print!("Current time: {}", time_test);
    println!();
}
*/
#[tokio::main]
async fn main() {
    test_run().await
}
