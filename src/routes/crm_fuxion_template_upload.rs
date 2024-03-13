use std::collections::HashSet;
use std::fmt::Display;
use std::io;
use std::io::{Cursor, Write};
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, put, Responder, web};
use actix_web::web::Bytes;
use calamine::{DataType, open_workbook, Reader, Xlsx};
use chrono::{Local, Utc};
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

#[post("/pedidos_delivery")]
async fn cargar_archivos_delivery(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {

    //Únicamente para el corte.
    let mut corte =String::new();
    if let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(name) = content_type.get_name() {
            match name {
                "corte" => {
                    let bytes = field.next().await.unwrap().unwrap(); // Obtiene Bytes
                    corte = String::from_utf8_lossy(&bytes).to_string(); // Convierte a String
                    println!("Arrived: {}", corte);
                }
                _ => (),
            }
        }
    }

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
                            None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };
                        let valor_columna_2 = match row.get(2) {
                            Some(valor) => valor.to_string(),
                            None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_3 = match row.get(3) {
                            Some(valor) => valor.to_string(),
                            None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_4 = match row.get(4) {
                            Some(valor) => valor.to_string(),
                            None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_5 = match row.get(5) {
                            Some(valor) => valor.to_string(),
                            None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                        };

                        let valor_columna_6 = match row.get(6) {
                            Some(valor) => valor.to_string(),
                            None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
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
        if let Some(segundo_elemento) = fila.get(5) {
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
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_7 = match row.get(7) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_9 = match row.get(9) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_6 = match row.get(6) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    //Orden de compra: 10314073
                                    let valor_columna_archivo_1 = match fila.get(0) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_archivo_2 = match fila.get(1) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_3 = match fila.get(2) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_4 = match fila.get(3) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_5 = match fila.get(4) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };
                                    let valor_columna_archivo_6 = match fila.get(5) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
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

    // Para el pedido

    // Eliminar filas duplicadas basadas en el primer elemento
    let mut cuartos_elementos = HashSet::new();
    let mut matriz_sin_duplicados = Vec::new();

    for fila in &matriz {
        let orden_compra_elemento = fila[4].clone();
        if !cuartos_elementos.contains(&orden_compra_elemento) {
            matriz_sin_duplicados.push(fila.clone());
            cuartos_elementos.insert(orden_compra_elemento);
        }
    }


    println!("Orden Log Consola");
    // Imprimir la matriz sin filas duplicadas
    for fila in &matriz_sin_duplicados {
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!();
    }


    println!("Detalle Orden Log Consola");

// Para el detalle del pedido
// Iteramos sobre cada fila de la matriz
    for fila in &matriz {
// Iteramos sobre cada elemento de la fila e imprimimos
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!(); // Agregamos un salto de línea después de imprimir cada fila
    }


    //GUARDAMOS EN LA BASE DE DATOS
    // Establecer la conexión fuera del bucle
    let mut connection = establish_connection().await.unwrap();
    let now = Utc::now();

    // Formatear la fecha y hora en el formato deseado.
    let formatted_date_time = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();


    for fila in &matriz_sin_duplicados {
        let query = format!(r#"INSERT INTO WMS_EC.dbo.TD_CR_PEDIDO (NUM_PEDIDO, PROCEDENCIA, FECHA,
                                     CTE, --1 CONSOLIDADO 2 DELIVERY

                                     CTE_PROCEDE, CONTACTO, TEL_CONTACTO, TIPO,
                                     CANTIDAD, SUBTOTAL, IMPUESTO, TOTAL, PROVINCIA,
                                     CANTON, --Campo de corte del excel
                                     DISTRITO, CVECIUDAD,
                                     DIRECCION_REF, OBSERVACIONES, ESTATUS, FEC_ALTA, FEC_MODIF, ORIGEN_PEDIDO, URGENTE,
                                     FEC_DESPACHO, COD_VENDEDOR, PERSONA_RECIBE)
        VALUES ({}, 7182, N'{}', 2, 7182, N'{}', N'{}', 0, 1, 0, 0, 0,
        N'TEST', N'{}', N'TEST', N'FUX_UIO_EC', N'', N'', N'N', N'{}',
        N'{}', N'Andrea Salomé Ibarra Morillo', 0, null, N'', N'');"#, fila[4], formatted_date_time, fila[0], fila[6], corte, formatted_date_time, formatted_date_time);


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


    //Pedido Detalle
    let mut contadorPD = 0; // Inicializamos el contador en 0

    for filaPD in matriz {
        contadorPD += 1;

        let query = format!(r#"INSERT INTO WMS_EC.dbo.TD_CR_PEDIDO_DET (NUM_PEDIDO, PROCEDENCIA, ARTICULO, ART_PROCEDE,
                                         LINEA, --Consecutivo para los items
                                         CANTIDAD, TOTAL, PRECIO,
                                         IMPUESTO, CAMPANIA, ART_PACK_NOLOGICO)
        VALUES ({}, 7182, {}, 7182, {}, {}, 0, 0, 0, N'', 0);"#, filaPD[4], filaPD[1], contadorPD, filaPD[2]);

        // Ejecutar la consulta dentro del bucle
        if let Err(err) = sqlx::query(&*query)
            .execute(&mut connection)
            .await
        {

            // Imprimir la fila que causó el error
            println!("Fila que causó el error: {:?}", filaPD);

            eprintln!("Error al insertar en la base de datos: {}", err);
            // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
        }
    }


    // --Add esto es importante
    for filaCENTRA in matriz_sin_duplicados {
        let query = format!(r#"INSERT INTO WMS_EC.dbo.TR_CR_PEDIDO_CENTRALIZADO (NUM_PEDIDO, PROCEDENCIA, ORDEN_COMPRA,
                                                  PEDIDO_CLIENTE, --Orden primaria
                                                  PERMITE_CENTRA,
                                                  CENTRALIZADO, REMISION)
VALUES ({}, 7182, N'', N'{}', 0, N'', 0);"#, filaCENTRA[4], filaCENTRA[5]);


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


#[post("/pedidos_consolidado")]
async fn cargar_archivos_consolidado(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {

    //Únicamente para el corte.
    let mut corte =String::new();
    if let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();

        if let Some(name) = content_type.get_name() {
            match name {
                "corte" => {
                    let bytes = field.next().await.unwrap().unwrap(); // Obtiene Bytes
                    corte = String::from_utf8_lossy(&bytes).to_string(); // Convierte a String
                    println!("Arrived: {}", corte);
                }
                _ => (),
            }
        }
    }

    // Creamos un vector vacío de strings
    let mut vector_consolidado: Vec<Vec<String>> = Vec::new();

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
            println!("...................");

            let mut concatenado_nombre_apellido = "0".to_string();
            let mut phone_destinatario = "0".to_string();
            let mut nueva_columna = "0".to_string();
            let mut nueva_columna_guia = "0".to_string();

            // Itera sobre cada fila del rango, ignorando la primera fila de encabezados.
            for (row_index, row) in range.rows().skip(1).enumerate() {
                // Extrae el valor de la primera celda de la fila actual.
                if let Some(cell_nombre) = row.get(0) {
                    if let Some(cell_apellido) = row.get(1) {
                        if let Some(cell_phone) = row.get(5) {
                            // Extrae el valor de la sexta celda (índice 6) de la fila actual.
                            if let Some(valor) = row.get(6) {
                                // Extrae el valor de la séptima celda (índice 7) de la fila actual.
                                if let Some(valor_siete) = row.get(7) {
                                    // Verifica si la primera celda y la sexta celda no están vacías.
                                    if !cell_nombre.is_empty() && !valor.is_empty() {
                                        // Si la primera celda no está vacía, asigna el valor de la sexta celda a la nueva_columna.
                                        nueva_columna = valor.to_string();
                                    }

                                    // Imprime el valor de la nueva columna (valor de la sexta celda).
                                    println!("Cell: {}", nueva_columna);

                                    // Verifica si la primera celda y la séptima celda no están vacías.
                                    if !cell_nombre.is_empty() && !valor_siete.is_empty() {
                                        //NombreDestinatario
                                        //ApellidoDestinatario
                                        concatenado_nombre_apellido = format!("{} {}", cell_nombre, cell_apellido);

                                        //phoneDestinatario
                                        phone_destinatario = cell_phone.to_string();

                                        // Si la primera celda no está vacía, asigna el valor de la séptima celda a la nueva_columna_guia.
                                        nueva_columna_guia = valor_siete.to_string();
                                    }

                                    // Imprime el valor de la nueva columna guía (valor de la séptima celda).
                                    println!("Cell: {}", nueva_columna_guia);

                                    // Verifica si la primera celda está vacía, la sexta celda tiene datos y la séptima celda tiene datos.
                                    if cell_nombre.is_empty() && !valor.is_empty() && !nueva_columna_guia.is_empty() {
                                        // Crea una nueva fila con los valores de la sexta celda, la nueva columna y la nueva columna guía.
                                        let nueva_fila: Vec<String> = vec![cell_nombre.to_string(), valor.to_string(), nueva_columna.clone(), nueva_columna_guia.clone(), concatenado_nombre_apellido.clone(), phone_destinatario.clone()];

                                        // Agrega la nueva fila al vector consolidado.
                                        vector_consolidado.push(nueva_fila);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("No se encontró el primer archivo en la carga útil multipart.");
    }

    // Iteramos sobre cada fila de la matriz
    for fila in &vector_consolidado {
        // Iteramos sobre cada elemento de la fila e imprimimos
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!(); // Agregamos un salto de línea después de imprimir cada fila
    }


    //==========================SEGUNDO ARCHIVO - Consolidado===========================


    // Matriz para almacenar las filas que coinciden
    let mut matched_rows_aux = Vec::new();

    // Creamos un vector vacío para representar la matriz
    let mut matriz_consolidado_tabla_dos: Vec<Vec<String>> = Vec::new();

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
            println!("1ra Columna: ID Empresario");
            //Validación primera columna
            for (row_index, row) in range.rows().skip(4).enumerate() {
                if let Some(cell) = row.get(6) {
                    // Verificar si la celda contiene datos -- Muy importante
                    if !cell.is_empty() {
                        //println!("{:?}", cell);

                        // Itera sobre cada fila del vector e imprime el primer elemento (primera columna)
                        for fila in &vector_consolidado {

                            //La primera columna columna se encuentra con un espacio en blango
                            if let Some(primer_elemento) = fila.get(1) {
                                //println!("{}", primer_elemento);

                                if primer_elemento.contains(&cell.to_string()) {
                                    // Si hay una coincidencia, agregamos la fila a la matriz de coincidencias
                                    matched_rows_aux.push(row_index);

                                    // Extraemos los valores de los Option<&DataType> y los convertimos a String
                                    let valor_columna_1 = match row.get(1) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_7 = match row.get(7) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_9 = match row.get(9) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_6 = match row.get(6) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };


                                    //Orden de compra: 10314073
                                    let valor_columna_archivo_1 = match fila.get(0) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_archivo_2 = match fila.get(1) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    let valor_columna_archivo_3 = match fila.get(2) {
                                        Some(valor) => valor.to_string(),
                                        None => "-".to_string(), // Puedes establecer un valor por defecto si es necesario
                                    };

                                    // if let Some(primer_elemento) = fila.get(1) {
                                    //     println!("ORDEN PRIMARIA: {}", primer_elemento);
                                    // }

                                    // Agregamos una fila a la matriz
                                    let nueva_fila: Vec<String> = vec![valor_columna_1, valor_columna_7, valor_columna_9, valor_columna_6, valor_columna_archivo_1, valor_columna_archivo_2, valor_columna_archivo_3];
                                    matriz_consolidado_tabla_dos.push(nueva_fila);
                                }
                            }
                        }
                    } else {
                        // Si la celda está vacía, se puede detener el bucle
                        break;
                    }
                }
            }
        }
    } else {
        println!("No se encontró el segundo archivo en la carga útil multipart.");
    }


    print!("INICIO");
    // Iteramos sobre cada fila de la matriz
    for fila in &matriz_consolidado_tabla_dos {
        // Iteramos sobre cada elemento de la fila e imprimimos
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!(); // Agregamos un salto de línea después de imprimir cada fila
    }
    print!("FIN");


    // Imprimimos las filas coincidentes
    println!("Filas coincidentes:");
    for row_index in matched_rows_aux {
        println!("Fila {}", row_index + 1); // Sumamos 1 para obtener el número de fila basado en la indexación del usuario
    }


    // Para el pedido

    // Eliminar filas duplicadas basadas en el primer elemento
    let mut cuartos_elementos = HashSet::new();
    let mut matriz_sin_duplicados = Vec::new();

    for fila in &matriz_consolidado_tabla_dos {
        let orden_compra_elemento = fila[3].clone();
        if !cuartos_elementos.contains(&orden_compra_elemento) {
            matriz_sin_duplicados.push(fila.clone());
            cuartos_elementos.insert(orden_compra_elemento);
        }
    }


    println!("Orden Log Consola");
    // Imprimir la matriz sin filas duplicadas
    for fila in &matriz_sin_duplicados {
        for elemento in fila {
            print!("{} ", elemento);
        }
        println!();
    }



    //GUARDAMOS EN LA BASE DE DATOS
    // Establecer la conexión fuera del bucle
    let mut connection = establish_connection().await.unwrap();
    let now = Utc::now();

    // Formatear la fecha y hora en el formato deseado.
    let formatted_date_time = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

    for fila in &matriz_sin_duplicados {
        let query = format!(r#"INSERT INTO WMS_EC.dbo.TD_CR_PEDIDO (NUM_PEDIDO, PROCEDENCIA, FECHA,
                                     CTE, --1 CONSOLIDADO 2 DELIVERY

                                     CTE_PROCEDE, CONTACTO, TEL_CONTACTO, TIPO,
                                     CANTIDAD, SUBTOTAL, IMPUESTO, TOTAL, PROVINCIA,
                                     CANTON, --Campo de corte del excel
                                     DISTRITO, CVECIUDAD,
                                     DIRECCION_REF, OBSERVACIONES, ESTATUS, FEC_ALTA, FEC_MODIF, ORIGEN_PEDIDO, URGENTE,
                                     FEC_DESPACHO, COD_VENDEDOR, PERSONA_RECIBE)
        VALUES ({}, 7182, N'{}', 1, 7182, N'{}', N'{}', 0, 1, 0, 0, 0,
        N'TEST', N'{}', N'TEST', N'FUX_UIO_EC', N'', N'', N'N', N'{}',
        N'{}', N'Andrea Salomé Ibarra Morillo', 0, null, N'', N'');"#, fila[3], formatted_date_time, fila[0], fila[5], corte, formatted_date_time, formatted_date_time);

        // Ejecutar la consulta dentro del bucle
        if let Err(err) = sqlx::query(&*query)
            .execute(&mut connection)
            .await
        {

            // Imprimir la fila que causó el error
            println!("Fila que causó el error: {:?}", fila);

            eprintln!("Error al insertar en la base de datos: {}", err);
            // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
        }
    }

    //DETALLE DEL PEDIDO
    let mut contadorPD = 0; // Inicializamos el contador en 0

    for filaPD in matriz_consolidado_tabla_dos {
        contadorPD += 1;

        let query = format!(r#"INSERT INTO WMS_EC.dbo.TD_CR_PEDIDO_DET (NUM_PEDIDO, PROCEDENCIA, ARTICULO, ART_PROCEDE,
                                         LINEA, --Consecutivo para los items
                                         CANTIDAD, TOTAL, PRECIO,
                                         IMPUESTO, CAMPANIA, ART_PACK_NOLOGICO)
        VALUES ({}, 7182, {}, 7182, {}, {}, 0, 0, 0, N'', 0);"#, filaPD[3], filaPD[1], contadorPD, filaPD[2]);

        // Ejecutar la consulta dentro del bucle
        if let Err(err) = sqlx::query(&*query)
            .execute(&mut connection)
            .await
        {

            // Imprimir la fila que causó el error
            println!("Fila que causó el error: {:?}", filaPD);

            eprintln!("Error al insertar en la base de datos: {}", err);
            // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
        }
    }


    // --Add esto es importante
    for filaCENTRA in matriz_sin_duplicados {
        let query = format!(r#"INSERT INTO WMS_EC.dbo.TR_CR_PEDIDO_CENTRALIZADO (NUM_PEDIDO, PROCEDENCIA, ORDEN_COMPRA,
                                                  PEDIDO_CLIENTE, --Orden primaria
                                                  PERMITE_CENTRA,
                                                  CENTRALIZADO, REMISION)
VALUES ({}, 7182, N'', N'{}', 0, N'', 0);"#, filaCENTRA[3], filaCENTRA[6]);


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

    pub fn config(conf: &mut web::ServiceConfig) {
        let scope = web::scope("/api/fuxion")
            .service(cargar_archivos_delivery)
            .service(cargar_archivos_consolidado)
            ;

        conf.service(scope);
    }


