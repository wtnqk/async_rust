use reqwest::Error;
use std::time::Instant;

//tokio async
#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let start_time = Instant::now();

    let (_, _, _, _) = tokio::join!(
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
        reqwest::get(url),
    );

    let elapsed_time = start_time.elapsed();
    println!("Request took {} ,s", elapsed_time.as_millis());
    Ok(())
}

// use tokio async, but this is sequence
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let url = "https://jsonplaceholder.typicode.com/posts/1";
//     let start_time = Instant::now();
//
//     let first = reqwest::get(url);
//     let second = reqwest::get(url);
//     let third = reqwest::get(url);
//     let fourth = reqwest::get(url);
//
//     let first = first.await?;
//     let second = second.await?;
//     let third = third.await?;
//     let fourth = fourth.await?;
//
//     let elapsed_time = start_time.elapsed();
//     println!("Request took {} ,s", elapsed_time.as_millis());
//     Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let url = "https://jsonplaceholder.typicode.com/posts/1";
//     let start_time = Instant::now();
//     let _ = reqwest::get(url).await?;
//
//     let elapsed_time = start_time.elapsed();
//     println!("Request took {} ,s", elapsed_time.as_millis());
//     Ok(())
// }

// use std::thread;
// multi fibonnaci
// fn main() {
//     let mut threads = Vec::new();
//     for i in 0..8 {
//         let handle = thread::spawn(move || {
//             let result = fibonacci(4000);
//             println!("Thread {} result: {}", i, result);
//         });
//         threads.push(handle);
//     }
//     for handle in threads {
//         handle.join().unwrap();
//     }
// }
//
// fn fibonacci(n: u64) -> u64 {
//     if n == 0 || n == 1 {
//         return n;
//     }
//     fibonacci(n - 1) + fibonacci(n - 2)
// }
