use async_once::AsyncOnce;
use sea_orm::{Database, DatabaseConnection};

lazy_static! {
    pub static ref DB: AsyncOnce<DatabaseConnection> = AsyncOnce::new(async {
        Database::connect("postgres://postgres:example@pg_db/longhu")
            .await
            .expect("db init failed")
    });
}
