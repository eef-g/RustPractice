struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

pub fn cars_showcase()
{
    println!("\n\n-=| Cars Showcase |=-\n");
    println!("Making 3 cars!");
    let mut car1 = car_factory(String::from("Red"), Transmission::Manual, false);
    let mut car2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    let mut car3 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 1 is a {} car.", car1.color);
    println!("Car 2 is a {} car.", car2.color);
    println!("Car 3 is a {} car.", car3.color);
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0,
    };
    return car;
}
