mod group;
use group::test_run;

#[tokio::main]
async fn main() {
    test_run().await
}
