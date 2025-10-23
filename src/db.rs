/*
Code connects to the database using an asynchronoous
connection pool. It prints a success message for
successful connection or generates error otherwise


*/

//update based on database used
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn userdb_connection() ->Result<(), sqlx::Error>{
	//defines the database URL
	let db_url = "sqliteusers.db"; //needs to be update

	//create a connection pool
	let pool = SqlitePoolOptions::new()
    .max_connections(5) //needs to update with the number of connection
    .connect(&db_url)
    .await?;

	println!("Successfully Connected to the Sqlite Database");

	Ok(())
}