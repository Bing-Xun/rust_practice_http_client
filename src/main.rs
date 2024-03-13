use tokio::time::{sleep, Duration};

async fn task1() {
    println!("Task 1 started");
    sleep(Duration::from_secs(5)).await;
    println!("Task 1 completed");
}

async fn task2() {
    println!("Task 2 started");
    sleep(Duration::from_secs(1)).await;
    println!("Task 2 completed");
}

async fn main_task() {
    println!("Main task started");

    // 启动异步任务，并交错执行
    let join_handle1 = tokio::spawn(task1());
    let join_handle2 = tokio::spawn(task2());

    println!("##### ##### #####");

    // 等待任务完成
    println!("## 1");
    // join_handle1.await.unwrap();
    println!("## 2");
    join_handle2.await.unwrap();
    println!("## 3");

    println!("Main task completed");
}

#[tokio::main]
async fn main() {
    main_task().await;
}
