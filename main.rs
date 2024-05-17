struct Coffee {
milk_type: Some(Milk::Oat),
sugar_count:u8,
}

enum Milk {
Dairy,
Oat,
Soy

}

fn main(){
    let morning_latte: Coffee = Coffee {
        milk_type: Some(Milk::Oat),
        sugar_count: 4,
    };
    let espresso: Coffee {
        milk_type: None,
        sugar_count: 4,
    };
}