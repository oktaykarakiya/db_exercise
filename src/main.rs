mod seeder;
mod retriever;
use std::io::{self, Write};



#[tokio::main] 
async fn main(){
        loop {
            print!("Please enter 'seed' to seed the database or 'retrieve' to retrieve data from the database: ");
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => {
                    let input = user_input.trim();
                    if input == "exit" {
                        println!("Exiting program.");
                        break;
                    }
                    if input == "seed" {
                        println!("Seeding database.");
                        seeder::seed().await;
                    }
                    if input == "retrieve" {
                        println!("Retrieving data from database.");
                        retriever::retrieve_data().await;  
                    }
                },
                Err(error) => println!("Error reading your input: {}", error),
            }
        }
}



