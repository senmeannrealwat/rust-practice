//! TODO: get the code to compile by **re-ordering** the statements
//!  in the `example` function. You're not allowed to change the
//!  `spawner` function nor what each line does in `example`.
//!   You can wrap existing statements in blocks `{}` if needed.
use std::sync::Arc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Arc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
