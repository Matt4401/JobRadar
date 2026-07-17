// all about pass word

pub async fn establish_connection() {
    let database_url = get_env_variable("DATABASE_URL");
    let mut db = Database::new(&database_url).await;

    match db.connect().await {
        Ok(_) => println!("Database connection established successfully."),
        Err(e) => println!("Error establishing database connection: {}", e),
    }
}

pub async fn get_history() {

}

// all first job :
// connection
// recuperation des histories
// users verifications
// creation db
// creation tables

// all about update thanks to migrationsScript
