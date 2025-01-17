use std::thread;
use std::time::{Duration, Instant, SystemTime};

fn main() {
    // 获取当前系统时间
    let now = SystemTime::now();
    println!("Current system time: {:?}", now);

    // 计算时间差
    let later = SystemTime::now();
    match later.duration_since(now) {
        Ok(duration) => println!("Duration since now: {:?}", duration),
        Err(e) => println!("Error: {:?}", e),
    }

    // 时间测量
    let start = Instant::now();
    thread::sleep(Duration::from_secs(2));
    let elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);

    // 自定义时间间隔
    let custom_duration = Duration::new(5, 0); // 5 秒
    println!("Custom duration: {:?}", custom_duration);

    // 时间相加
    let new_duration = elapsed + custom_duration;
    println!("Combined duration: {:?}", new_duration);

    // 时间相减
    if new_duration > elapsed {
        let difference = new_duration - elapsed;
        println!("Difference: {:?}", difference);
    } else {
        println!("Invalid subtraction");
    }

    // 将 Duration 转换为秒数（浮点数）
    let seconds = new_duration.as_secs_f64();
    println!("Duration in seconds: {}", seconds);

    // 将秒数转换为 Duration
    let back_to_duration = Duration::from_secs_f64(seconds);
    println!("Back to duration: {:?}", back_to_duration);
}