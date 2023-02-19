use migration::sea_orm as so;

#[derive(Clone)]
pub struct State {
    pub database_connection: so::DatabaseConnection,
    pub default_thumbnail: String,
    pub public_directory: String,
}