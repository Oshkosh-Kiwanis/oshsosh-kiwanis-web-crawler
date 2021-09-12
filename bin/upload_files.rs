/// Upload the generated json files to google cloud storage
/// so we can then do some reporting on them in datastudio

use std::{error::Error, time::Duration};

use chrono::Utc;
use cloud_storage::{Client};
use tokio::time::{interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we have to get the default client credentials
    let client = Client::default();

    let bucket = "new-top-dog";
    let mime_type = "text/csv";

    let mut interval = interval(Duration::from_secs(60));
    loop {
        interval.tick().await;
        println!("[TICK]");
        // read the json files as bytes and then save them in the cloud storage bucket
        let top_dogs_file = "top-dogs.csv";
        let contest_goals_file= "contest-goals.csv";

        // we don't want to fail if we are unable to read the file
        // so just skip the rest of the iteration if an error occured
        let dogs_file_buf = match std::fs::read(top_dogs_file) {
            Ok(buf) => buf,
            Err(e) => {
                println!("[ERROR]: Unable to read file; file={}; error={}", top_dogs_file, e);
                continue;
            }
        };
        let contests_file_buf = match std::fs::read(contest_goals_file) {
            Ok(buf) => buf,
            Err(e) => {
                println!("[ERROR]: Unable to read file; file={}; error={}", contest_goals_file, e);
                continue;
            }
        };

        let top_dog_filename = format!("top-dogs-{}.csv", Utc::now().timestamp());
        let contest_goals_filename = format!("contest-goals-{}.csv", Utc::now().timestamp());
        client.object().create(bucket, dogs_file_buf, &top_dog_filename, mime_type).await?;
        println!("[INFO]: Upload successful; file={}", top_dogs_file);
        client.object().create(bucket, contests_file_buf, &contest_goals_filename, mime_type).await?;
        println!("[INFO]: Upload successful; file={}", contest_goals_file);

        // we don't really care if the remove file fails
        let _ = std::fs::remove_file(top_dogs_file);
        println!("[INFO]: Removed file; file={}", top_dogs_file);
        let _ = std::fs::remove_file(contest_goals_file);
        println!("[INFO]: Removed file; file={}", contest_goals_file);

        println!("[DONE]");
    }
}