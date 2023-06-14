use std::sync::{Condvar, Arc};

use tokio::{net::TcpListener, sync::{RwLock, Mutex, Barrier, Semaphore}, fs::{OpenOptions, File}, io::{AsyncReadExt, AsyncWriteExt}};



#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(0));

    let task1 = {
        let shared_data = Arc::clone(&shared_data);
        tokio::spawn(async move {
            for _ in 0..5 {
                let mut data = shared_data.lock().await;
                *data += 1;
                println!("Task 1: {}", *data);
                drop(data); 
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        })
    };

    let task2 = {
        let shared_data = Arc::clone(&shared_data);
        tokio::spawn(async move {
            for _ in 0..5 {
                let mut data = shared_data.lock().await;
                *data += 1;
                println!("Task 2: {}", *data);
                drop(data); 
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        })
    };

    tokio::try_join!(task1, task2).unwrap();
}
// #[tokio::main]
// async fn main() {
//     //ex
//     tokio::task::spawn(write_file());
//     tokio::task::spawn(read_file());

//     //mutex ex
//     let file_mutex = Arc::new(Mutex::new(get_lock_file().await));
//     tokio::task::spawn(write_file_lock(file_mutex.clone())).await.unwrap();
//     tokio::task::spawn(read_file_lock(file_mutex)).await.unwrap();


//     //tokio::task::yield_now().await;
//     tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
// }


// async fn read_file() -> String {
//     let mut file = get_file().await;
//     let mut str = String::new();
//     file.read_to_string(&mut str).await.unwrap();
//     println!("{:?}",str);
//     str
// }

// async fn write_file() {
//     let mut file = get_file().await;
//     let new_buffer = "write";
//     file.write(new_buffer.as_bytes()).await.unwrap();
//     file.flush().await.unwrap();
//     println!("Done");
// }

// async fn get_file() -> File {
//     let file = OpenOptions::new()
//     .read(true)
//     .write(true)
//     .open("file.txt")
//     .await
//     .unwrap();
//     file
// }

// async fn get_lock_file() -> File {
//     let file = OpenOptions::new()
//     .read(true)
//     .write(true)
//     .open("file-lock.txt")
//     .await
//     .unwrap();
//     file
// }

// async fn read_file_lock(file_mutex: Arc<Mutex<File>>) -> String {
//     let mut file = file_mutex.lock().await;
//     let mut str = String::new();
//     file.read_to_string(&mut str).await.unwrap();
//     println!("{:?}",str);
//     drop(file);
//     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//     str
// }

// async fn write_file_lock(file_mutex: Arc<Mutex<File>>) {
//     let mut file = file_mutex.lock().await;
//     let new_buffer = "write";
//     file.write_all(new_buffer.as_bytes()).await.unwrap();
//     file.flush().await.unwrap();
//     println!("Done");
//     drop(file);
//     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
// }
