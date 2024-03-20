use std::sync::{Arc, Mutex};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let mtx = Arc::new(Mutex::new(0));
    // both tasks are run concurrently on a single thread - this is key to the deadlock
    tokio::join!(work(mtx.clone()), work(mtx.clone()));

    // Solution 1 : Double the runtime threads
    // tokio::spawn(async { work(&mtx).await });
    println!("{}", *mtx.lock().unwrap());
}

async fn work(mtx: Arc<Mutex<i32>>) {
    let id = std::thread::current().id();
    println!("Task id {:?} started", id);
    {
        let mut _v = mtx.lock().unwrap();
        println!("Task id: {:?} locked mutex", id);
        // slow network request
        // Note the .await. A std::sync::Mutex lock is held across this .await
        // Let the two tasks be T1 and T2
        // When T1 locks the thread, it's suspended due to this network request
        // T2 can now progress on this runtime thread, but it blocks on l:16 `let mut _v = mtx.lock()`
        // This makes the runtime think that T2 is progressing, when it's actually waiting for the blocking
        // mtx unlock, and T1 cannot resume because T2 is taking up the entire thread
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    println!("Task id: {:?} unlocked mutex", id);
}
