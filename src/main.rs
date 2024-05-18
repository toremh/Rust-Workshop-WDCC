#[derive(serde::Deserialize)]
enum CoffeeType{
    FlatWhite,
    Espresso,
    Frappe
}

#[derive(serde::Deserialize)]
enum Milk {
    Oat,
    Soy,
    Pea
  }

  #[derive(serde::Deserialize)]
  struct Coffee {
    coffee_type: CoffeeType,
    milk: Option<Milk>,
    is_decaf: bool,
    sugar_count: u8
}


trait Decaffeinatable {
    fn remove_caffeine(&mut self);
}

impl Decaffeinatable for Coffee {
    fn remove_caffeine(&mut self){
        self.is_decaf = true;
    }
}

// fn main(){
//     let mut morning_coffee = Coffee {
//         coffee_type: CoffeeType::FlatWhite,
//         milk: Some(Milk::Oat),
//         sugar_count: 1,
//         is_decaf: false
//       };
//       morning_coffee.remove_caffeine();



// }

use axum::{routing::get, Router};

#[tokio::main]
async fn main(){
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/",get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World"
}