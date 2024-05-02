use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub struct PgConnector {
    postgres_url: String,
}
impl PgConnector {
    pub fn new(postgres_url: String) -> Self {
        PgConnector { postgres_url }
    }

    pub async fn open_connection(&self) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.postgres_url)
            .await
    }

    pub async fn test(&self) -> Result<(), Error> {
        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.postgres_url)
            .await?;

        connection.close().await;

        Ok(())
    }
}
