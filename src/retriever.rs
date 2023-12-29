use bson::doc;
use futures_util::StreamExt;
use mongodb::Client;
use serde_json;

pub struct Orders {
    pub id: i32,
    pub name: String,
}

pub async fn retrieve_data() {
    let client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .expect("Failed to initialize standalone client.");

    let db = client.database("test");

    let orders = db.collection::<Orders>("Orders");

        let pipeline = vec![
        doc! {
            "$lookup": {
                "from": "Boards",
                "localField": "id",
                "foreignField": "order",
                "as": "BoardDetails"
            }
        },
        doc! { "$unwind": "$BoardDetails" },

        doc! {
            "$lookup": {
                "from": "Defects",
                "localField": "BoardDetails.id",
                "foreignField": "board",
                "as": "DefectDetails"
            }
        },
        doc! {
            "$project": {
                "order_id": "$id",
                "order_name": "$name",
                "board_id": "$BoardDetails.id",
                "board_datetime": { "$toDate": "$BoardDetails.datetime" },
                "board_length_mm": "$BoardDetails.length_mm",
                "has_defect": { "$gt": [{ "$size": "$DefectDetails" }, 0] }
            }
        },
        doc! { "$sort": { "order_id": 1, "board_datetime": 1 } },
        doc! {
            "$setWindowFields": {
                "partitionBy": "$order_id",  // Assuming you want the calculation within each order
                "sortBy": { "board_datetime": 1 },
                "output": {
                    "previous_board_datetime": { "$shift": { "by": -1, "output": "$board_datetime" } }
                }
            }
        },
        doc! {
            "$group": {
                "_id": "$order_id",
                "order_id": { "$first": "$order_id" },
                "order_name": { "$first": "$order_name" },
                "order_start": { "$min": "$board_datetime" },
                "order_end": { "$max": "$board_datetime" },
                "no_boards": { "$sum": 1 },
                "no_boards_with_defects": { "$sum": { "$cond": ["$has_defect", 1, 0] } },
                "sum_length_mm": { "$sum": "$board_length_mm" },
                "avg_length_mm": { "$avg": "$board_length_mm" },
                "time_interval_to_previous_order_minutes": { "$avg": { "$subtract": ["$board_datetime", "$previous_board_datetime"] } },
                "stdev_length_mm": { "$stdDevSamp": "$board_length_mm" },
                "avg_time_interval_between_boards_milliseconds": {
                "$avg": {
                    "$cond": [
                        { "$eq": ["$previous_board_datetime", 0] }, // Check if previous_board_datetime is null
                        0, // If null (i.e., the first document), set interval to 0
                        { "$subtract": ["$board_datetime", "$previous_board_datetime"] } // Else calculate the difference
                    ]
                }
            },
            }
        },
        doc! {
            "$setWindowFields": {
                "partitionBy": null,
                "sortBy": { "order_start": 1 },
                "output": {
                    "previous_order_end": { "$shift": { "output": "$order_end", "by": -1, "default": null } }
                }
            }
        },

        doc! {
            "$project": {
                "_id": 0,
                "order_id": 1,
                "order_name": 1,
                "no_boards": 1,
                "no_boards_with_defects": 1,
                "order_start": { "$dateToString": { "format": "%Y-%m-%dT%H:%M:%S.%LZ", "date": "$order_start", "timezone": "UTC" } },
                "order_end": { "$dateToString": { "format": "%Y-%m-%dT%H:%M:%S.%LZ", "date": "$order_end", "timezone": "UTC" } },
                "sum_length_mm": { "$toInt": "$sum_length_mm", },
                "avg_length_mm": { "$toInt": "$avg_length_mm", },
                "stdev_length_mm": { "$cond": { "if": { "$eq": [{ "$trunc": [ { "$toDouble": "$stdev_length_mm" }, 7] }, null] }, "then": 0, "else": { "$trunc": [ { "$toDouble": "$stdev_length_mm" }, 7] } } },
                "time_interval_to_previous_order_minutes": { "$toInt": {
                    "$round": [
                        {
                            "$cond": {
                                "if": { "$eq": ["$previous_order_end", null] },
                                "then": null,
                                "else": { "$divide": [{ "$subtract": ["$order_start", "$previous_order_end"] }, 1000 * 60] }
                            }
                        },
                        0
                    ]
                }},
                "avg_time_interval_between_boards_milliseconds": 
                    {
                        "$cond": {
                            "if": { "$eq": ["$avg_time_interval_between_boards_milliseconds", null] },
                            "then": 0,
                            "else": { "$toInt":  "$avg_time_interval_between_boards_milliseconds" }
                        }
                    },
            }
        },
        //{doc! {"$limit": 2}},
        doc! { "$sort": { "order_id": 1 } }
    ];



    // Execute the aggregation query
    let cursor = orders
        .aggregate(pipeline, None)
        .await
        .expect("Failed to execute aggregation.");

    // Process the results
    cursor
        .for_each(|doc| async {
            // Make the closure async
            match doc {
                Ok(document) => {
                    println!("{}", serde_json::to_string_pretty(&document).unwrap());
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        })
        .await;
}
