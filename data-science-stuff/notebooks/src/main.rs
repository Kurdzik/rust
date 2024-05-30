extern crate diesel;
   mod db;
   mod models;
   mod schema;

   fn main() {
       let connection = db::establish_connection();
       println!("Connected to the database.");
       // Further code to manipulate or query the database
   }