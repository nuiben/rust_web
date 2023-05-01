use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use mongodb::bson::doc;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

   // Load the MongoDB connection string from an environment variable:
   let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
   let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
   let client = Client::with_options(options)?;
   let games = client.database("game_backlog").collection("titles");
   let mut game_titles: Vec<String> = Vec::new();
   let file = File::open("sample.txt");

   //Push each line of the txt file to a vector stack.
   if let Ok(file) = file {
      let reader = BufReader::new(file);
      for line in reader.lines() {
         if let Ok(game) = line {
            game_titles.push(game);
         }
      }
   }

   //Iterate through our newly made vector called game_titles,
   // the doc macro places the String title into a BSON format.
   for (i, game) in game_titles.iter().enumerate() {
      let new_document = doc! {
         "title": game,
      };
      println!("inserted {} - {}", i, new_document); //left for testing so you don't have to insert to MongoDB
      //games.insert_one(new_document.clone(), None).await?;
   }

   Ok(())
}
