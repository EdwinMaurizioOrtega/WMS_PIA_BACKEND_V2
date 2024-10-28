use sqlx::Connection;
use sqlx::mssql::MssqlConnection;
use tokio_util::codec::{Decoder};

pub async fn establish_connection() -> Result<MssqlConnection, sqlx::Error> {
    MssqlConnection::connect("mssql://sati:12345qwert@192.168.0.143:53078/WMS_EC").await
}

// // Función para establecer la conexión con la base de datos
// pub async fn establish_connection_v2() -> Result<Client<tokio_util::compat::Compat<TcpStream>>, anyhow::Error> {
//     let mut config = Config::new();
//
//     // Configura la autenticación, los parámetros del servidor, la instancia y la base de datos
//     config.host("192.168.0.143");
//     config.port(53078);
//     config.authentication(AuthMethod::sql_server("sati", "12345qwert"));
//     config.instance_name("CWMSPRD"); // Especifica la instancia
//     config.database("WMS_EC"); // Especifica la base de datos
//     // config.trust_cert(); // Omite la verificación de certificados
//
//     // Crea un TcpStream conectado a la dirección configurada
//     let addr = config.get_addr();
//     let tcp_result = TcpStream::connect(addr)
//         .await
//         .context("Failed to connect to the TCP stream");
//
//     let tcp = match tcp_result {
//         Ok(tcp) => tcp,
//         Err(e) => {
//             eprintln!("Error connecting to the TCP stream: {:?}", e);
//             return Err(anyhow::anyhow!("Failed to connect to the TCP stream: {:?}", e));
//         }
//     };
//
//     // Configura TCP_NODELAY
//     if let Err(e) = tcp.set_nodelay(true) {
//         eprintln!("Failed to set TCP_NODELAY: {:?}", e);
//         return Err(anyhow::anyhow!("Failed to set TCP_NODELAY: {:?}", e));
//     }
//
//     // Intentar conectar el cliente y manejar el error
//     let client_result = Client::connect(config, tcp.compat_write()).await;
//
//     match client_result {
//         Ok(client) => Ok(client),
//         Err(e) => {
//             eprintln!("Error connecting the client to SQL Server: {:?}", e);
//             Err(anyhow::anyhow!("Failed to connect the client to SQL Server: {:?}", e))
//         }
//     }
// }