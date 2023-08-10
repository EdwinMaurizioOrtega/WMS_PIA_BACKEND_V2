use sqlx::Connection;
use sqlx::mssql::MssqlConnection;

pub async fn establish_connection() -> Result<MssqlConnection, sqlx::Error> {
    MssqlConnection::connect("mssql://sati:12345qwert@192.168.0.143:53078/WMS_EC").await
}
