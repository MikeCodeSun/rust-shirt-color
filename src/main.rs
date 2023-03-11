

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Shirt {
    inventory: Vec<ShirtColor>
}

impl Shirt {
    fn give_away(&self, person_prefer: Option<ShirtColor>) -> ShirtColor {
        person_prefer.unwrap_or_else(|| {
            self.own_most()
        })
    }

    fn own_most(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.inventory {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let shirts = Shirt {
        inventory: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Blue]
    };
    let user_p1 = Some(ShirtColor::Red);
    let user1_prefer = shirts.give_away(user_p1);
    println!("User 1 like {:?}, and get {:?}", user_p1, user1_prefer);
    

    let user_p2 = None;
    let user2_prefer = shirts.give_away(user_p2);
    println!("User 1 like {:?}, and get {:?}", user_p2, user2_prefer);
}
