use mongodb::Client;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Boards {
    pub id: i32,
    pub order: i32,
    pub datetime: String,
    pub length_mm: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Defects {
    pub board: i32,
    pub description: String,
}

pub async fn seed() {

    let client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .expect("Failed to initialize standalone client.");

    let db = client.database("test");
    let orders = db.collection::<Orders>("Orders");
    let boards = db.collection::<Boards>("Boards");
    let defects = db.collection::<Defects>("Defects");

    let orders_data = vec![
        Orders {
            id: 1,
            name: "Order 1".to_string(),
        },
        Orders {
            id: 2,
            name: "Order 2".to_string(),
        },
        Orders {
            id: 3,
            name: "Order 3".to_string(),
        },
        Orders {
            id: 4,
            name: "Order 4".to_string(),
        },
    ];

    let boards_data = vec![
        Boards {
            id: 11,
            order: 1,
            datetime: "2023-01-01 07:00:00.000".to_string(),
            length_mm: 4200,
        },
        Boards {
            id: 8,
            order: 1,
            datetime: "2023-01-01 07:00:00.430".to_string(),
            length_mm: 4250,
        },
        Boards {
            id: 10,
            order: 1,
            datetime: "2023-01-01 07:00:01.150".to_string(),
            length_mm: 4180,
        },
        Boards {
            id: 3,
            order: 1,
            datetime: "2023-01-01 07:00:03.110".to_string(),
            length_mm: 4060,
        },
        Boards {
            id: 2,
            order: 2,
            datetime: "2023-01-01 07:15:23.500".to_string(),
            length_mm: 3520,
        },
        Boards {
            id: 4,
            order: 2,
            datetime: "2023-01-01 07:15:25.123".to_string(),
            length_mm: 3580,
        },
        Boards {
            id: 9,
            order: 2,
            datetime: "2023-01-01 07:15:25.670".to_string(),
            length_mm: 3610,
        },
        Boards {
            id: 1,
            order: 2,
            datetime: "2023-01-01 07:15:26.330".to_string(),
            length_mm: 3840,
        },
        Boards {
            id: 5,
            order: 3,
            datetime: "2023-01-01 08:23:15.680".to_string(),
            length_mm: 5200,
        },
        Boards {
            id: 7,
            order: 3,
            datetime: "2023-01-01 08:23:16.030".to_string(),
            length_mm: 5220,
        },
        Boards {
            id: 6,
            order: 4,
            datetime: "2023-01-01 08:50:01.330".to_string(),
            length_mm: 4060,
        },
    ];

    let defects_data = vec![
        Defects {
            board: 4,
            description: "Bark".to_string(),
        },
        Defects {
            board: 4,
            description: "Black knot".to_string(),
        },
        Defects {
            board: 4,
            description: "Wane".to_string(),
        },
        Defects {
            board: 6,
            description: "Knot".to_string(),
        },
        Defects {
            board: 8,
            description: "Bark".to_string(),
        },
        Defects {
            board: 8,
            description: "Knot".to_string(),
        },
        Defects {
            board: 10,
            description: "Split".to_string(),
        },
    ];

    // Insert the data into the database
    match orders.insert_many(orders_data, None).await {
        Ok(result) => println!("Inserted {} orders", result.inserted_ids.len()),
        Err(e) => println!("Error inserting orders: {}", e),
    };

    match boards.insert_many(boards_data, None).await {
        Ok(result) => println!("Inserted {} boards", result.inserted_ids.len()),
        Err(e) => println!("Error inserting boards: {}", e),
    };

    match defects.insert_many(defects_data, None).await {
        Ok(result) => println!("Inserted {} boards", result.inserted_ids.len()),
        Err(e) => println!("Error inserting boards: {}", e),
    };
}
