use std::time::Duration;
use std::pin::Pin;

fn main() {
    trpl::run(async {
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }

        // handle.await.unwrap();

        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
            
        let fut1 = async move {
            for i in 1..10 {
                tx1.send(format!("Task A: section {i}")).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            }
        };

        let tx2 = tx.clone();
        
        let fut2 = async move {
            for i in 1..5 {
                tx2.send(format!("Task B: section {i}")).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            }
        };
            
            
            // rx/ tx futures 

        
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
                ];
                
                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(50)).await;
                }
            };
            
            let rx_fut = async {
                
                // while not None
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };
                
            let fut_vec: Vec<Pin<Box<dyn Future<Output = ()>>>> =
                vec![Box::pin(fut1), Box::pin(fut2), Box::pin(tx_fut), Box::pin(rx_fut)];

            trpl::join_all(fut_vec).await;


    });



}