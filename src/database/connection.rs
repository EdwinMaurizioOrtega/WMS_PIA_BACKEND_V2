use sqlx::Connection;
use sqlx::mssql::MssqlConnection;

pub async fn establish_connection() -> Result<MssqlConnection, sqlx::Error> {
    MssqlConnection::connect("mssql://sati:12345qwert@192.168.0.143:53078/WMS_EC").await
}



// pub async fn establish_connectionV2() -> Result<SqlConnection, tiberius::error::Error> {
//     let tcp_stream = TcpStream::connect("192.168.0.143:53078").await?;
//     let connection = SqlConnection::connect(
//         tcp_stream,
//         "sati",
//         "12345qwert",
//         "WMS_EC",
//     )
//         .await?;
//
//     println!("Connected to the database!");
//     Ok(connection)
// }


