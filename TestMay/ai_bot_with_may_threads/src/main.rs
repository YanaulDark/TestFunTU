use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        return "Result";
    });

    let result = handle.await.unwrap();
    println!("{}", result);

}
