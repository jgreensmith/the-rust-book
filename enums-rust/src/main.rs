#[derive(Debug)]

struct DldInProg {
    size: f32
}
impl DldInProg {
    fn percentage_done(&self, done: f32) -> i32 {
        (100.0 / self.size * done).ceil() as i32
    }
}

#[derive(Debug)]

enum ClientState {
    Downloading(DldInProg),
    Downloaded,
    Cloud

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) if i >= 10 => Some(i),
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let ten = plus_one(Some(10));

    let state = ClientState::Downloading(DldInProg { size: 60.0 });
    let downloaded: f32 = 30.0;
    if let ClientState::Downloading(prog) = state {
        assert_eq!(prog.percentage_done(downloaded), 50);
        println!("{}% Downloaded", prog.percentage_done(downloaded));
    }


    assert_eq!(six, Some(6));
    assert_eq!(none, None);
    assert_eq!(ten, Some(10));

    println!("{}", six.is_some_and(|x| x > 2))
}
