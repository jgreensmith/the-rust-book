use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..3 {
            println!("hi number {i} from the spawned thread!");
            //thread::sleep(Duration::from_millis(1));
        }
    });
    // thread::spawn(|| {
    //     for i in 1..3 {
    //         println!("hi number {i} from 2nd spawned thread!");
    //         //thread::sleep(Duration::from_millis(1));
    //     }
    // });

    for i in 1..3 {
        println!("hi number {i} from the main thread!");
        //thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    println!("Before defining closure: {list:?}");
    
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();


}