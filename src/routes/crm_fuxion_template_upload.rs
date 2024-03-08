use std::fmt::Display;
use std::io;
use std::io::{Cursor, Write};
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, put, Responder, web};
use actix_web::web::Bytes;
use calamine::{open_workbook, Reader, Xlsx};
use chrono::Local;
use futures::{AsyncReadExt, StreamExt};
use serde_json::json;
use sqlx::Row;
use tempfile::{NamedTempFile, tempfile};
use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::{MC_WEB_PROVINCIAS_CIUDADES, McClienteCnt, McClienteCntAux};
use crate::models::mc_consolidado::{MC_WEB_CONSOLIDADO_CARGA_PEDIDOS, PedidoConsolidado};
use crate::models::user_model::User;


// Función para limpiar el valor
fn limpiar_valor<T: Display>(valor: &Option<T>) -> String {
    match valor {
        Some(inner) => {
            let valor_str = format!("{}", inner);
            valor_str
                .replace("Float(", "")
                .replace("String(\"", "")
                .replace("\")", "")
                .replace(".0)", "")
        }
        None => String::new(),
    }
}


// Función para limpiar una cadena
fn limpiar_cadenaV2(cadena: &str) -> String {
    cadena.replace("Float(", "")
        .replace("String(\"", "")
        .replace("\")", "")
        .replace(".0)", "")
}

#[post("/pedidos")]
async fn cargar_dos_archivos(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {


    // Creamos un vector vacío de strings
    let mut vector: Vec<Vec<String>> = Vec::new();

    //Primer archivo - Mecanizado
    if let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(filename) = content_type.get_filename() {
            println!("Nombre del primer archivo recibido: {}", filename);
        } else {
            println!("No se pudo obtener el nombre del primer archivo.");
        }


        //1. Creación de un archivo temporal usando tempfile::NamedTempFile:
        // NamedTempFile::new()?: Esto crea un nuevo archivo temporal y devuelve un Result que puede contener el archivo temporal (Ok(NamedTempFile)) o un error (Err).
        let mut temp_file = NamedTempFile::new()?;
        let mut file_content = Vec::new();

        //2. Lectura y escritura de bytes en el archivo temporal:
        // field.next().await: Obtiene el siguiente fragmento de bytes del campo multipart.
        // let data = chunk?;: Desempaqueta el fragmento de bytes o maneja un error si ocurre.
        //     file_content.extend_from_slice(&data);: Extiende el vector file_content con los bytes del fragmento actual.
        //     temp_file.write_all(&data)?;: Escribe los bytes del fragmento en el archivo temporal.
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_content.extend_from_slice(&data);
            temp_file.write_all(&data)?;
        }

        //3. Obtención de la ruta del archivo temporal:
        // temp_file.path(): Obtiene la ruta del archivo temporal.
        //     to_owned(): Crea una copia propia de la ruta.
        let temp_file_path = temp_file.path().to_owned();

        //4. Procesamiento del archivo Excel con calamine:
        // open_workbook(&temp_file_path): Intenta abrir el archivo Excel en la ruta proporcionada.
        //     .expect("Cannot open file"): Proporciona un mensaje de error si no puede abrir el archivo.
        let mut workbook: Xlsx<_> = open_workbook(&temp_file_path).expect("Cannot open file");

        //5. Limpieza y eliminación automática del archivo temporal:
        // El archivo temporal se eliminará automáticamente al salir del bloque de alcance.
        // Al utilizar NamedTempFile, el archivo temporal se eliminará automáticamente cuando la variable temp_file salga del alcance.


        //if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_INDIRECTOS") {
        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {

            //Número de filas
            let mut totalFilas: i32 = 0;
            println!("2da Columna: ORDEN  DE COMPRA");
            //Validación primera columna
            let mut boolean_validacion_individual_1: Vec<bool> = vec![];
            for (row_index, row) in range.rows().skip(1).enumerate() {
                if let Some(cell) = row.get(1) {
                    // Verificar si la celda contiene datos -- Muy importante
                    if !cell.is_empty() {
                        println!("{:?}", cell);


                        let valor_columna_1 = match row.get(1) {
                            Some(valor) => valor.to_string(),
                            None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };
                        let valor_columna_2 = match row.get(2) {
                            Some(valor) => valor.to_string(),
                            None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_3 = match row.get(3) {
                            Some(valor) => valor.to_string(),
                            None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_4 = match row.get(4) {
                            Some(valor) => valor.to_string(),
                            None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_5 = match row.get(5) {
                            Some(valor) => valor.to_string(),
                            None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_6 = match row.get(6) {
                            Some(valor) => valor.to_string(),
                            None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let nueva_fila: Vec<String> = vec![valor_columna_1, valor_columna_2, valor_columna_3, valor_columna_4, valor_columna_5, valor_columna_6];

                        // Agregamos algunos strings al vector
                        vector.push(nueva_fila);

                        //Saber el número de filas.
                        totalFilas += 1;
                        boolean_validacion_individual_1.push(true);
                    } else {
                        // Si la celda está vacía, se puede detener el bucle
                        break;
                    }
                }
            }

            println!("Número total de filas: {}", totalFilas);
        }
    } else {
        println!("No se encontró el primer archivo en la carga útil multipart.");
    }


    //========Muy importante para buscar en el segundo archivo.

    // Itera sobre cada fila del vector e imprime el segundo elemento (segunda columna)
    for fila in &vector {
        if let Some(segundo_elemento) = fila.get(1) {
            println!("{}", segundo_elemento);
        }
    }


    //==========================SEGUNDO ARCHIVO - Consolidado===========================


    // Matriz para almacenar las filas que coinciden
    let mut matched_rows = Vec::new();

    // Creamos un vector vacío para representar la matriz
    let mut matriz: Vec<Vec<String>> = Vec::new();

    if let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(filename) = content_type.get_filename() {
            println!("Nombre del segundo archivo recibido: {}", filename);
        } else {
            println!("No se pudo obtener el nombre del segundo archivo.");
        }


        //1. Creación de un archivo temporal usando tempfile::NamedTempFile:
        // NamedTempFile::new()?: Esto crea un nuevo archivo temporal y devuelve un Result que puede contener el archivo temporal (Ok(NamedTempFile)) o un error (Err).
        let mut temp_file = NamedTempFile::new()?;
        let mut file_content = Vec::new();

        //2. Lectura y escritura de bytes en el archivo temporal:
        // field.next().await: Obtiene el siguiente fragmento de bytes del campo multipart.
        // let data = chunk?;: Desempaqueta el fragmento de bytes o maneja un error si ocurre.
        //     file_content.extend_from_slice(&data);: Extiende el vector file_content con los bytes del fragmento actual.
        //     temp_file.write_all(&data)?;: Escribe los bytes del fragmento en el archivo temporal.
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            file_content.extend_from_slice(&data);
            temp_file.write_all(&data)?;
        }

        //3. Obtención de la ruta del archivo temporal:
        // temp_file.path(): Obtiene la ruta del archivo temporal.
        //     to_owned(): Crea una copia propia de la ruta.
        let temp_file_path = temp_file.path().to_owned();

        //4. Procesamiento del archivo Excel con calamine:
        // open_workbook(&temp_file_path): Intenta abrir el archivo Excel en la ruta proporcionada.
        //     .expect("Cannot open file"): Proporciona un mensaje de error si no puede abrir el archivo.
        let mut workbook: Xlsx<_> = open_workbook(&temp_file_path).expect("Cannot open file");

        //5. Limpieza y eliminación automática del archivo temporal:
        // El archivo temporal se eliminará automáticamente al salir del bloque de alcance.
        // Al utilizar NamedTempFile, el archivo temporal se eliminará automáticamente cuando la variable temp_file salga del alcance.

        if let Some(Ok(range)) = workbook.worksheet_range_at(0) {

            //Número de filas
            let mut totalFilas: i32 = 0;
            println!("1ra Columna: ID Empresario");
            //Validación primera columna
            let mut boolean_validacion_individual_1: Vec<bool> = vec![];
            for (row_index, row) in range.rows().skip(4).enumerate() {
                if let Some(cell) = row.get(6) {
                    // Verificar si la celda contiene datos -- Muy importante
                    if !cell.is_empty() {
                        println!("{:?}", cell);
                        //Saber el número de filas.
                        totalFilas += 1;
                        boolean_validacion_individual_1.push(true);

                        // Itera sobre cada fila del vector e imprime el primer elemento (primera columna)
                        for fila in &vector {
                            if let Some(primer_elemento) = fila.first() {
                                println!("{}", primer_elemento);


                                if primer_elemento.contains(&cell.to_string()) {
                                    // Si hay una coincidencia, agregamos la fila a la matriz de coincidencias
                                    matched_rows.push(row_index);

                                    // Extraemos los valores de los Option<&DataType> y los convertimos a String
                                    let valor_columna_1 = match row.get(1) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_7 = match row.get(7) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_9 = match row.get(9) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_6 = match row.get(6) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_archivo_1 = match fila.get(1) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_archivo_2 = match fila.get(2) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_3 = match fila.get(3) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_4 = match fila.get(4) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_5 = match fila.get(5) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_6 = match fila.get(6) {
                                        Some(valor) => valor.to_string(),
                                        None => "".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    // if let Some(primer_elemento) = fila.get(1) {
                                    //     println!("ORDEN PRIMARIA: {}", primer_elemento);
                                    // }

                                    // Agregamos una fila a la matriz
                                    let nueva_fila: Vec<String> = vec![valor_columna_1, valor_columna_7, valor_columna_9, valor_columna_6, valor_columna_archivo_1, valor_columna_archivo_2, valor_columna_archivo_3, valor_columna_archivo_4, valor_columna_archivo_5, valor_columna_archivo_6];
                                    matriz.push(nueva_fila);
                                }
                            }
                        }
                    } else {
                        // Si la celda está vacía, se puede detener el bucle
                        break;
                    }
                }
            }

            println!("Número total de filas: {}", totalFilas);
        }
    } else {
        println!("No se encontró el segundo archivo en la carga útil multipart.");
    }


// Imprimimos las filas coincidentes
    println!("Filas coincidentes:");
    for row_index in matched_rows {
        println!("Fila {}", row_index + 1); // Sumamos 1 para obtener el número de fila basado en la indexación del usuario
    }

// Iteramos sobre cada fila de la matriz
    for fila in &matriz {
// Iteramos sobre cada elemento de la fila e imprimimos
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!(); // Agregamos un salto de línea después de imprimir cada fila
    }


    //GUARDAMOS EN LA BASE DEDATOS



    // Establecer la conexión fuera del bucle
    let mut connection = establish_connection().await.unwrap();


    for fila in &matriz {

    let query = format!(r#"INSERT INTO WMS_EC.dbo.TD_CR_PEDIDO (NUM_PEDIDO, PROCEDENCIA, FECHA,
                                     CTE, --1 CONSOLIDADO 2 DELIVERY

                                     CTE_PROCEDE, CONTACTO, TEL_CONTACTO, TIPO,
                                     CANTIDAD, SUBTOTAL, IMPUESTO, TOTAL, PROVINCIA,
                                     CANTON, --Campo de corte del excel
                                     DISTRITO, CVECIUDAD,
                                     DIRECCION_REF, OBSERVACIONES, ESTATUS, FEC_ALTA, FEC_MODIF, ORIGEN_PEDIDO, URGENTE,
                                     FEC_DESPACHO, COD_VENDEDOR, PERSONA_RECIBE)
VALUES ({}, 7182, N'2024-03-08 15:40:54.000', 1, 7182, N'Ponce Ureña, Sandra Lucia', N'729385585', 0, 1, 0, 0, 0,
        N'TEST', N'CORTE 1', N'TEST', N'FUX_UIO_EC', N'', N'', N'N', N'2024-03-08 15:40:54.060',
        N'2024-03-08 15:40:54.060', N'Andrea Salomé Ibarra Morillo', 0, null, N'', N'');"#, fila[0]);




        // Limpiar cada valor en la fila


        // Ejecutar la consulta dentro del bucle
        if let Err(err) = sqlx::query(&*query)
            .execute(&mut connection)
            .await
        {
            eprintln!("Error al insertar en la base de datos: {}", err);
            // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
        }
    }













// Aquí puedes devolver una respuesta HTTP adecuada, dependiendo de tu lógica de negocio.
    Ok(HttpResponse::Ok().finish())
}

// Función para imprimir la matriz
fn imprimir_matriz(matriz: &Vec<Vec<String>>) {
    for fila in matriz {
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!();
    }
}


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/fuxion")
        .service(cargar_dos_archivos)
        ;

    conf.service(scope);
}


