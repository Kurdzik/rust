use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use tokio;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Get database URL from an environment variable
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Use SQL query to get column names from a specific table
    let columns = sqlx::query(
        // Replace 'your_table' with the actual name of your table
        r#"
        SELECT column_name
        FROM information_schema.columns
        WHERE table_schema = 'public'
        AND table_name   = 'your_table'
        "#
    )
    .fetch_all(&pool)
    .await?;

    // Print out column names
    for column in columns {
        let name: String = column.get("column_name");
        println!("Column name: {}", name);
    }

    Ok(())
}