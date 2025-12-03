fn main() {
    examples();
}

fn examples() {
    // create an instance of coffee
    let mocha = Coffee {
        price: 9.99,
        name: String::from("Mocha"),
        is_hot: true,
    };

    // print instance properties
    println!(
        "My {} coffee is hot? {} and costs {}",
        mocha.name, mocha.is_hot, mocha.price
    );

    // use fn to create instance
    let redbull = create_coffee(String::from("Red Bull"), 5.99, false);

    println!(
        "{} and {} and {}",
        redbull.name, redbull.price, redbull.is_hot
    );

    drink_cofee(&redbull);

    println!("{:?}", redbull);

    let mut car = Car {
        name: String::from("Lambo"),
        year: 2010,
    };

    // follow car display and update process
    car.display_car();
    car.update_year();
    car.display_car();

    // use constructor
    let toyota = Car::new(String::from("Toyota"), 2020);
    toyota.display_car();
}

#[derive(Debug)]
struct Car {
    name: String,
    year: u32,
}

impl Car {
    // create a constructor
    fn new(name: String, year: u32) -> Self {
        Car { name, year }
    }

    // this way self does not take ownership of the instance
    fn display_car(&self) {
        println!("Car model: {}", self.name);
        println!("Car year: {}", self.year);
    }

    fn update_year(&mut self) {
        self.year = 1999
    }
}

// defining coffee struct deriving debug trait
#[derive(Debug)]
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

// fn that returns a coffee
fn create_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    return Coffee {
        name,
        price,
        is_hot,
    };
}

// fn accesses the address
fn drink_cofee(coffee: &Coffee) {
    println!("Drinking my delicious {}", coffee.name);
}
