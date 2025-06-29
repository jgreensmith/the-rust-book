
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn is_rectangle(&self) -> bool {
        let x: u32 = self.width;
        let y: u32 = self.height;
        x != y && x > 0 && y > 0
    }
}

struct Winner {
    result: bool,
    self_area: u32,
    other_area: u32
}

impl Winner {
    fn by_how_much(&self) -> u32 {
        let a = self.self_area;
        let b = self.other_area;
        if a > b {a - b} else {b - a}
    }
}

impl Rectangle {
    fn is_bigger(&self, other: &Rectangle) -> Winner {
        let self_area: u32 = self.width * self.height;
        let other_area: u32 = other.width * other.height;

        let mut who_won = Winner {
            result: false,
            self_area,
            other_area
        };

        if self_area > other_area {
            who_won.result = true;
        };
        who_won
    }
}
    

fn main() {
    let rect1 = Rectangle {
        width: 3,
        height: 2,
    };
    let rect2 = Rectangle {
        width: 7,
        height: 30,
    };
    
    if !rect1.is_rectangle() || !rect2.is_rectangle() {
        println!("One of those is not a rectangle, this is only for rectangles you cheeky wee scamp");
        return;
    };

    println!("Is rect1: {:#?}\n\nbigger than rect2: {:#?}??", &rect1, &rect2);
 
    let winner = rect1.is_bigger(&rect2);
    if winner.result {
        println!("Yes rect1 is bigger by {}", winner.by_how_much());
    } else {
        println!("No rect2 is bigger by {}", winner.by_how_much());
    }  

    // println!("{}", rect1.is_rectangle());
    // println!("{:#?}", rect1);
    

}