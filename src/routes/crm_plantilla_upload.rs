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
use tempfile::{NamedTempFile, tempfile};
use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::{MC_WEB_PROVINCIAS_CIUDADES, McClienteCnt, McClienteCntAux};
use crate::models::mc_consolidado::{MC_WEB_CONSOLIDADO_CARGA_PEDIDOS, PedidoConsolidado};


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

#[post("/pedido_puntual")]
async fn cargar_validar_file_pedidos_puntuales(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();


        if let Some(name) = content_type.get_name() {
            match name {
                "fileCNT" => {
                    let filename = content_type.get_filename().unwrap();

                    println!("Nombre de alchivo CNT: {}", filename);

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


                    //if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_PUNTUALES") {
                    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {

                        //Validación todas las columnas
                        let mut boolean_validacion_all: Vec<bool> = vec![];

                        //Número de filas
                        let mut totalFilas: i32 = 0;
                        println!("1ra Columna: N° BODEGA OPEN");
                        //Validación primera columna
                        let mut boolean_validacion_individual_uno: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if let Some(cell) = row.get(0) {
                                // Verificar si la celda contiene datos -- Muy importante
                                if !cell.is_empty() {
                                    println!("{:?}", cell);
                                    //Saber el número de filas.
                                    totalFilas += 1;
                                    boolean_validacion_individual_uno.push(true);
                                } else {
                                    // Si la celda está vacía, se puede detener el bucle
                                    break;
                                }
                            }
                        }

                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_uno);

                        if boolean_validacion_individual_uno.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_uno.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        //Validar - ESTATUS DE BODEGA/ABIERT/CERRADO/ETC - C.L. SAP DIRECTO
                        println!("2da Columna: BODEGA SAP");
                        let mut boolean_validacion_individual_dos: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(1) {
                                println!("{:?}", cell);

                                let mut connection = establish_connection().await.unwrap();

                                let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", cell);

                                let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                    .fetch_all(&mut connection)
                                    .await;

                                match cli {
                                    Ok(clientes) => {

                                        // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                        if let Some(primer_cliente) = clientes.get(0) {
                                            println!("Estado Cliente: {:?}", primer_cliente.ESTADO);

                                            // 0 CERRADO 1 ABIERTO 2 TEMPORAL
                                            if primer_cliente.ESTADO == 1 {
                                                println!("El local se encuentra abierto.");
                                                boolean_validacion_individual_dos.push(true);
                                            } else {
                                                println!("El cliente se encuentra cerrado.");
                                                boolean_validacion_individual_dos.push(false);
                                            }
                                        } else {
                                            println!("La consulta no devolvió ningún cliente");
                                            boolean_validacion_individual_dos.push(false);
                                        }
                                    }
                                    Err(err) => {
                                        println!("No se encuentra");
                                        //HttpResponse::InternalServerError().body(err.to_string()),
                                        boolean_validacion_individual_dos.push(false);
                                    }
                                }
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_dos);

                        if boolean_validacion_individual_dos.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true_dos = boolean_validacion_individual_dos.iter().all(|&x| x == true);

                            if todos_true_dos {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        println!("3ra Columna: CODIGO SAP");
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(2) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_3.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        println!("4ta Columna: PRODUCTO");
                        let mut boolean_validacion_individual_4: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(3) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_4.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_4);

                        if boolean_validacion_individual_4.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_4.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        println!("5ta Columna: MARCA");
                        let mut boolean_validacion_individual_5: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(4) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_5.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_5);

                        if boolean_validacion_individual_5.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_5.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        println!("6ta Columna: DESCRIPCION");
                        let mut boolean_validacion_individual_6: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(5) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_6.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_6);

                        if boolean_validacion_individual_6.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_6.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        println!("7ma Columna: TIPO");
                        let mut boolean_validacion_individual_7: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(6) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_7.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_7);

                        if boolean_validacion_individual_7.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_7.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        println!("8va Columna: CANTIDAD");
                        let mut boolean_validacion_individual_8: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(7) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_8.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_8);

                        if boolean_validacion_individual_8.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_8.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        //Validar - CAMPO LLENO Y QUE SE REPITA LA INFO
                        println!("9na Columna: PEDIDO SAP");
                        let mut boolean_validacion_individual_9: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(8) {
                                let longitud = format!("{}", cell).len();
                                println!("Longitud: {}", longitud);

                                if longitud == 10 {
                                    //Pasa las validaciones
                                    println!("Pasa la validacion.");
                                    boolean_validacion_individual_9.push(true);
                                } else {
                                    println!("No pasa la validación.");
                                    boolean_validacion_individual_9.push(false);
                                }
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_9);

                        if boolean_validacion_individual_9.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_9.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        //Validar - CAMPO LLENO
                        println!("10ma Columna: ALMACEN EMISOR");
                        let mut boolean_validacion_individual_10: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(9) {
                                let longitud = format!("{}", cell).len();
                                println!("Longitud: {}", longitud);

                                if longitud == 4 {
                                    //Pasa las validaciones
                                    println!("Pasa la validación.");
                                    boolean_validacion_individual_10.push(true);
                                } else {
                                    println!("No pasa la validación.");
                                    boolean_validacion_individual_10.push(false);
                                }
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_10);

                        if boolean_validacion_individual_10.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_10.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        //Validar - NO OBLIGATORIO
                        println!("11va Columna: OBSERVACIONES");
                        let mut boolean_validacion_individual_11: Vec<bool> = vec![];

                        for (row_index, row) in range.rows().skip(2).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(10) {
                                println!("{:?}", cell);
                                boolean_validacion_individual_11.push(true);
                            }
                        }
                        println!("Número total de filas: {}", totalFilas);
                        println!("Validación individual: {:?}", boolean_validacion_individual_11);

                        if boolean_validacion_individual_11.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_11.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //==============================================================================

                        // Importante esta columna
                        // ALMACEN RECEPTOR
                        // 9000
                        // 9000
                        // 9000
                        // 9000
                        // 9000
                        // CAMPO FIJO

                        let numero_de_items = boolean_validacion_all.len();
                        println!("El número de items en el vector es: {}", numero_de_items);

                        let todos_true: bool = boolean_validacion_all.iter().all(|&x| x == true);

                        if todos_true {
                            println!("¡Todos los elementos son true!");


                            //Convertimos el excel en matriz para poder manipular todos los datos.

                            let mut matriz: Vec<Vec<Option<String>>> = Vec::new();

                            //Saltar dos filas recorrer hasta el total de filas anterior.
                            for (row_index, row) in range.rows().skip(2).take(totalFilas as usize).enumerate() {
                                let mut fila: Vec<Option<String>> = Vec::new();

                                for col_index in 0..=10 {
                                    if let Some(cell) = row.get(col_index) {
                                        let valor_como_string = format!("{:?}", cell);
                                        println!("{}", valor_como_string);
                                        fila.push(Some(valor_como_string));
                                    } else {
                                        fila.push(None);
                                    }
                                }

                                // Agregar la fila a la matriz
                                matriz.push(fila);

                                // Añadir una 11va columna con valor por defecto 9000
                                if let Some(fila) = matriz.last_mut() {
                                    fila.push(Some("9000".to_string()));
                                }

                                // ... Resto del código ...
                            }

                            println!("MATRIZ PEDIDOS PUNTUALES:");
                            println!("{:?}", matriz);

                            //print!("Imprimir Valores Matriz...");
                            // Imprimir la matriz con valores limpios
                            // for fila in &matriz {
                            //     for cell in fila {
                            //         if let Some(valor) = cell {
                            //             // Limpiar el formato Float("valor") y String("valor")
                            //             let valor_limpio = valor
                            //                 .replace("Float(", "")
                            //                 .replace("String(\"", "")
                            //                 .replace("\")", "")
                            //                 .replace(".0)", "");
                            //
                            //             // Imprimir el valor numérico o limpio
                            //             if let Ok(valor_numerico) = valor_limpio.parse::<f64>() {
                            //                 print!("{} ", valor_numerico);
                            //             } else {
                            //                 print!("{} ", valor_limpio);
                            //             }
                            //         } else {
                            //             print!("Empty ");
                            //         }
                            //     }
                            //     println!();
                            // }


                            //===================GUARDAMOS LOS DATOS EN LA DB===================

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "INSERT INTO MC_WEB_PEDIDOS_PUNTUALES (N_BODEGA_OPEN, BODEGA_SAP, CODIGO_SAP, PRODUCTO, MARCA, DESCRIPCION, TIPO, CANTIDAD, PEDIDO_SAP, ALMACEN_EMISOR, OBSERVACIONES, ALMACEN_RECEPTOR) VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12);";

                            for fila in &matriz {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_valor(valor)).collect();


                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .bind(&valores_limpios[11])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }


                            //Imprimir solo los valores de la columna uno
                            // for fila in &matriz {
                            //     if let Some(valor) = fila.get(0).and_then(|v| v.as_ref()) {
                            //         print!("{} ", valor);
                            //     } else {
                            //         print!("Empty ");
                            //     }
                            //     println!();
                            // }


                            //============== SEGUNDO BLOQUE CÓDIGO=============================

                            // Crear una matriz de 5x3 inicializada con "Empty"

                            //El numero 5 es una variable
                            let mut matrizMaster: Vec<Vec<String>> = vec![vec![String::from("Empty"); 20]; totalFilas as usize];

                            // Mostrar la matriz antes de la modificación
                            println!("Matriz original:");
                            imprimir_matriz(&matrizMaster);

                            // Obtener la fecha actual
                            let fecha_actual = Local::now();

                            // Formatear la fecha como "18/10/2023"
                            let fecha_formateada = fecha_actual.format("%d/%m/%Y").to_string();


                            for fila in &matriz {

                                // FECHA ACTUAL - FECHA
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[0] = fecha_formateada.to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //PEDIDO SAP - Pedido Traslado
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    //print!("{} ", valor);

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[1] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CAMPO FIJO - Centro Suministrador
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[2] = "CD06 CLD Telef Móvil Celistics".to_string();  // Asignar valores de la primera columna como cadenas
                                }


                                // ALMACEN EMISOR - Almacén emisor
                                if let Some(valor) = fila.get(9).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[3] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CODIGO SAP - Material
                                if let Some(valor) = fila.get(2).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[4] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }


                                //DESCRIPCION - Descripción Material
                                if let Some(valor) = fila.get(5).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[5] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CANTIDAD - Cantidad
                                if let Some(valor) = fila.get(7).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[6] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //BODEGA SAP - Centro
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[7] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //SI ES INDIRECTO DEL CAMPO "AGENTE COMERCIAL",CASO CONTRARIO CRUZO BODEGA SAP BASE CLIENTES "DESCRIPCION ALMACEN"
                                // Descripción Centro
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.DESCRIPCION_ALMACEN);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[8] = limpiar_cadenaV2(primer_cliente.DESCRIPCION_ALMACEN.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[8] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[8] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // COD CIUDAD
                                // CIUDAD
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT T1.*
                                                            FROM WMS_EC.dbo.MC_CLIENTE_CNT T0
                                                            INNER JOIN WMS_EC.dbo.MC_WEB_PROVINCIAS_CIUDADES T1 ON T1.ID_CIUDAD = T0.PROVINCIA
                                                            WHERE T0.CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<MC_WEB_PROVINCIAS_CIUDADES>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.NOMBRE_CIUDAD);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = limpiar_cadenaV2(primer_cliente.NOMBRE_CIUDAD.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO BLANCO
                                // GUIA COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[10] = "Empty".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // ALMACEN RECEPTOR
                                // Almacén
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[11] = "9000".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO FIJO
                                // COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[12] = "SERVIENTREGA".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // REGIONAL BASE CLIENTES
                                // CANAL
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.REGIONAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = limpiar_cadenaV2(primer_cliente.REGIONAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // PIA BASE CLIENTES
                                // COD.CLIENTE
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CVE);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = limpiar_cadenaV2(primer_cliente.CVE.to_string().as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // OBSERVACION
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[15] = "1".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CANAL BASE CLIENTES
                                // CATEGORÍA

                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CANAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = limpiar_cadenaV2(primer_cliente.CANAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO BLANCO
                                // ARTICULO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[17] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO FIJO
                                // ALMACEN

                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[18] = "30".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO BLANCO
                                // PEDIDO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[19] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                //FIN

                                println!("FIN");
                            }

                            // Mostrar la matriz después de la modificación
                            println!("MATRIZ CONSOLIDADO:");
                            imprimir_matriz(&matrizMaster);

                            //=====================GUARDAR LOS DATOS CONSOLIDADOS EN LA DB

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "insert into MC_WEB_CONSOLIDADO_CARGA_PEDIDOS (FECHA, PEDIDO_TRASLADO, CENTRO_SUMINISTRADOR, ALMACEN_EMISOR, MATERIAL,
                                              DESCRIPCION_MATERIAL, CANTIDAD, CENTRO, DESCRIPCION_CENTRO, CIUDAD,
                                              GUIA_COURIER, ALMACEN_RECEPTOR, COURIER, CANAL, COD_CLIENTE,
                                              OBSERVACION, CATEGORIA, ARTICULO, ALMACEN, PEDIDO)
                        values (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12, @p13, @p14, @p15, @p16, @p17, @p18, @p19, @p20);";

                            for fila in &matrizMaster {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_cadenaV2(valor)).collect();


                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .bind(&valores_limpios[11])
                                    .bind(&valores_limpios[12])
                                    .bind(&valores_limpios[13])
                                    .bind(&valores_limpios[14])
                                    .bind(&valores_limpios[15])
                                    .bind(&valores_limpios[16])
                                    .bind(&valores_limpios[17])
                                    .bind(&valores_limpios[18])
                                    .bind(&valores_limpios[19])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }

                            return Ok(HttpResponse::Ok().json("¡Archivo válido!"));
                        } else {
                            println!("No todos los elementos son true - Validar datos del archivo.");
                            return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                        }


                        //Fin.
                    } else {
                        println!("No se encontró la hoja 'PEDIDOS_PUNTUALES'");

                        return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                    }

                    // Asegurarse de que el archivo temporal se elimine cuando ya no se necesite
                }
                _ => (),
            }
        }
    }
    Ok(HttpResponse::Ok().json("Archivo procesado exitosamente")) // Puedes ajustar el mensaje según sea necesario
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

#[get("/consolidado")]
async fn reporte_consolidado() -> impl Responder {
    let mut connection = establish_connection().await.unwrap();

    let query = format!("SELECT * FROM MC_WEB_CONSOLIDADO_CARGA_PEDIDOS");

    let cli: Result<Vec<MC_WEB_CONSOLIDADO_CARGA_PEDIDOS>, sqlx::Error> = sqlx::query_as(&query)
        .fetch_all(&mut connection)
        .await;

    match cli {
        Ok(clientes) => HttpResponse::Ok().json(json!({"data": clientes})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/add_pedido_consolidado")]
async fn update_pedido_consolidado(data: web::Json<PedidoConsolidado>) -> impl Responder {
    println!("ID: {:?}", data.ID);

    let id = data.ID; // Extrayendo el valor del Option
    let pedido = &data.PEDIDO; // Extrayendo el valor del Option

    let mut connection = establish_connection().await.unwrap();

    let query = format!("UPDATE WMS_EC.dbo.MC_WEB_CONSOLIDADO_CARGA_PEDIDOS
    SET
    PEDIDO = '{}'
    WHERE ID = {:?};",
                        pedido,
                        id);

    println!("Generated SQL query: {}", query); // Imprimir la consulta SQL generada

    let result = sqlx::query(&query)
        .execute(&mut connection)
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "PEDIDO registrada correctamente."})),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//PEDIDO INDIRECTO
#[post("/pedido_indirecto")]
async fn cargar_validar_file_pedidos_indirectos(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();


        if let Some(name) = content_type.get_name() {
            match name {
                "fileCNT" => {
                    let filename = content_type.get_filename().unwrap();

                    println!("Nombre de alchivo CNT: {}", filename);

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

                        //Validación todas las columnas
                        let mut boolean_validacion_all: Vec<bool> = vec![];

                        //Número de filas
                        let mut totalFilas: i32 = 0;
                        println!("1ra Columna: CENTRO");
                        //Validación primera columna
                        let mut boolean_validacion_individual_1: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            if let Some(cell) = row.get(0) {
                                // Verificar si la celda contiene datos -- Muy importante
                                if !cell.is_empty() {
                                    println!("{:?}", cell);
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
                        println!("Validación individual: {:?}", boolean_validacion_individual_1);

                        if boolean_validacion_individual_1.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_1.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("2ra Columna: ALMACEN RECEPTOR");
                        //Validación primera columna
                        let mut boolean_validacion_individual_2: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(1) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_2.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_2);

                        if boolean_validacion_individual_2.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_2.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("3ra Columna: BODEGA SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(2) {
                                println!("{:?}", cell);

                                let mut connection = establish_connection().await.unwrap();

                                let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", cell);

                                let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                    .fetch_all(&mut connection)
                                    .await;

                                match cli {
                                    Ok(clientes) => {

                                        // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                        if let Some(primer_cliente) = clientes.get(0) {
                                            println!("Estado Cliente: {:?}", primer_cliente.ESTADO);

                                            // 0 CERRADO 1 ABIERTO 2 TEMPORAL
                                            if primer_cliente.ESTADO == 1 {
                                                println!("El local se encuentra abierto.");
                                                boolean_validacion_individual_3.push(true);
                                            } else {
                                                println!("El cliente se encuentra cerrado.");
                                                boolean_validacion_individual_3.push(false);
                                            }
                                        } else {
                                            println!("La consulta no devolvió ningún cliente");
                                            boolean_validacion_individual_3.push(false);
                                        }
                                    }

                                    Err(err) => {
                                        println!("No se encuentra");
                                        //HttpResponse::InternalServerError().body(err.to_string()),
                                        boolean_validacion_individual_3.push(false);
                                    }
                                }
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }

                        //Número de filas
                        println!("4ta Columna: AGENTE COMERCIAL");
                        //Validación primera columna
                        let mut boolean_validacion_individual_4: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(3) {
                                println!("{:?}", cell);

                                //Número de bodega SAP, la obtenemos del índice columna 2 - Válido de esta manera.
                                let aux_bodega_sap = row.get(2).unwrap();

                                let mut connection = establish_connection().await.unwrap();

                                let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", aux_bodega_sap);

                                let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                    .fetch_all(&mut connection)
                                    .await;

                                match cli {
                                    Ok(clientes) => {

                                        // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                        if let Some(primer_cliente) = clientes.get(0) {
                                            println!("DESCRIPCION_ALMACEN: {:?}", primer_cliente.DESCRIPCION_ALMACEN);
                                            boolean_validacion_individual_4.push(true);
                                        } else {
                                            println!("La consulta no devolvió ningún cliente");
                                            boolean_validacion_individual_4.push(false);
                                        }
                                    }

                                    Err(err) => {
                                        println!("No se encuentra");
                                        //HttpResponse::InternalServerError().body(err.to_string()),
                                        boolean_validacion_individual_4.push(false);
                                    }
                                }
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_4);

                        if boolean_validacion_individual_4.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_4.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("5ta Columna: CODIGO SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_5: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(4) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_5.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_5);

                        if boolean_validacion_individual_5.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_5.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }

                        //Número de filas
                        println!("6ta Columna: DESCRIPCION");
                        //Validación primera columna
                        let mut boolean_validacion_individual_6: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(5) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_6.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_6);

                        if boolean_validacion_individual_6.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_6.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("7ma Columna: CANTIDAD");
                        //Validación primera columna
                        let mut boolean_validacion_individual_7: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(6) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_7.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_7);

                        if boolean_validacion_individual_7.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_7.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("8va Columna: PEDIDO SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_8: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(7) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_8.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_8);

                        if boolean_validacion_individual_8.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_8.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("9na Columna: ICC");
                        //Validación primera columna
                        let mut boolean_validacion_individual_9: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(8) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_9.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_9);

                        if boolean_validacion_individual_9.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_9.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("10ma Columna: NO APLICA");
                        //Validación primera columna
                        let mut boolean_validacion_individual_10: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem
                            if let Some(cell) = row.get(9) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_10.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_10);

                        if boolean_validacion_individual_10.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_10.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }

                        //Número de filas
                        println!("11va Columna: FACTURA");
                        //Validación primera columna
                        let mut boolean_validacion_individual_11: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem

                            if let Some(cell) = row.get(10) {

                                // Verificar si la celda contiene datos -- Muy importante
                                if !cell.is_empty() {
                                    println!("{:?}", cell);
                                    boolean_validacion_individual_11.push(true);
                                } else {
                                    // Si la celda está vacía, se puede detener el bucle
                                    boolean_validacion_individual_11.push(false);
                                }
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_11);

                        if boolean_validacion_individual_11.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_11.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }


                        //Número de filas
                        println!("12va Columna: OBSERVACIONES");
                        //Validación primera columna
                        let mut boolean_validacion_individual_12: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(5).enumerate() {
                            println!("row_index: {}", row_index);
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            // Tu lógica de procesamiento para las filas del índice 0 al 3 ejem

                            if let Some(cell) = row.get(11) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_12.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_12);

                        if boolean_validacion_individual_12.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_12.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                                boolean_validacion_all.push(false);
                            }
                        }

                        //13va Columna
                        // ALMACEN EMISOR
                        // 1000
                        // CAMPO FIJO


                        let numero_de_items = boolean_validacion_all.len();
                        println!("El número de items en el vector es: {}", numero_de_items);

                        let todos_true: bool = boolean_validacion_all.iter().all(|&x| x == true);

                        if todos_true {
                            println!("¡Todos los elementos son true!");


                            //Convertimos el excel en matriz para poder manipular todos los datos.

                            let mut matriz: Vec<Vec<Option<String>>> = Vec::new();

                            //Saltar dos filas recorrer hasta el total de filas anterior.
                            for (row_index, row) in range.rows().skip(5).take(totalFilas as usize).enumerate() {
                                let mut fila: Vec<Option<String>> = Vec::new();
                                //Número de columnas de la matriz
                                for col_index in 0..=11 {
                                    if let Some(cell) = row.get(col_index) {
                                        let valor_como_string = format!("{:?}", cell);
                                        println!("{}", valor_como_string);
                                        fila.push(Some(valor_como_string));
                                    } else {
                                        fila.push(None);
                                    }
                                }

                                // Agregar la fila a la matriz
                                matriz.push(fila);

                                // Añadir una 13va columna con valor por defecto 1000
                                if let Some(fila) = matriz.last_mut() {
                                    fila.push(Some("1000".to_string()));
                                }

                                // ... Resto del código ...
                            }

                            println!("MATRIZ PEDIDOS INDIRECTOS:");
                            println!("{:?}", matriz);

                            println!("Imprimir Valores Matriz:");
                            //Imprimir la matriz con valores limpios
                            for fila in &matriz {
                                for cell in fila {
                                    if let Some(valor) = cell {
                                        // Limpiar el formato Float("valor") y String("valor")
                                        let valor_limpio = valor
                                            .replace("Float(", "")
                                            .replace("String(\"", "")
                                            .replace("\")", "")
                                            .replace(".0)", "");

                                        // Imprimir el valor numérico o limpio
                                        if let Ok(valor_numerico) = valor_limpio.parse::<f64>() {
                                            print!("{} ", valor_numerico);
                                        } else {
                                            print!("{} ", valor_limpio);
                                        }
                                    } else {
                                        print!("Empty ");
                                    }
                                }
                                println!();
                            }


                            //===================GUARDAMOS LOS DATOS EN LA DB===================

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "INSERT INTO MC_WEB_PEDIDOS_INDIRECTOS (CENTRO, ALMACEN_RECEPTOR, BODEGA_SAP, AGENTE_COMERCIAL, CODIGO_SAP,
                                       DESCRIPCION, CANTIDAD, PEDIDO_SAP, VAL1, VAL2, FACTURA, OBSERVACIONES,
                                       ALMACEN_EMISOR) VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12, @p13);";

                            for fila in &matriz {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_valor(valor)).collect();


                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .bind(&valores_limpios[11])
                                    .bind(&valores_limpios[12])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }


                            //Imprimir solo los valores de la columna uno
                            // for fila in &matriz {
                            //     if let Some(valor) = fila.get(0).and_then(|v| v.as_ref()) {
                            //         print!("{} ", valor);
                            //     } else {
                            //         print!("Empty ");
                            //     }
                            //     println!();
                            // }


                            //============== SEGUNDO BLOQUE CÓDIGO=============================

                            // Crear una matriz de 5x3 inicializada con "Empty"

                            //El numero 5 es una variable
                            let mut matrizMaster: Vec<Vec<String>> = vec![vec![String::from("Empty"); 20]; totalFilas as usize];

                            // Mostrar la matriz antes de la modificación
                            println!("Matriz original:");
                            imprimir_matriz(&matrizMaster);

                            // Obtener la fecha actual
                            let fecha_actual = Local::now();

                            // Formatear la fecha como "18/10/2023"
                            let fecha_formateada = fecha_actual.format("%d/%m/%Y").to_string();


                            for fila in &matriz {

                                // FECHA ACTUAL - FECHA
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[0] = fecha_formateada.to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //PEDIDO SAP - Pedido Traslado
                                if let Some(valor) = fila.get(7).and_then(|v| v.as_ref()) {
                                    //print!("{} ", valor);

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[1] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CAMPO FIJO - Centro Suministrador
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[2] = "CD06 CLD Telef Móvil Celistics".to_string();  // Asignar valores de la primera columna como cadenas
                                }


                                // ALMACEN EMISOR - Almacén emisor
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[3] = "1000".to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //CODIGO SAP - Material
                                if let Some(valor) = fila.get(4).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[4] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }


                                //DESCRIPCION - Descripción Material
                                if let Some(valor) = fila.get(5).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[5] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CANTIDAD - Cantidad
                                if let Some(valor) = fila.get(6).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[6] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //BODEGA SAP - Centro
                                if let Some(valor) = fila.get(2).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[7] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //SI ES INDIRECTO DEL CAMPO "AGENTE COMERCIAL",CASO CONTRARIO CRUZO BODEGA SAP BASE CLIENTES "DESCRIPCION ALMACEN"
                                // Descripción Centro
                                if let Some(valor) = fila.get(3).and_then(|v| v.as_ref()) {
                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[8] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // COD CIUDAD
                                // CIUDAD
                                if let Some(valor) = fila.get(2).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT T1.*
                                                            FROM WMS_EC.dbo.MC_CLIENTE_CNT T0
                                                            INNER JOIN WMS_EC.dbo.MC_WEB_PROVINCIAS_CIUDADES T1 ON T1.ID_CIUDAD = T0.PROVINCIA
                                                            WHERE T0.CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<MC_WEB_PROVINCIAS_CIUDADES>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.NOMBRE_CIUDAD);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = limpiar_cadenaV2(primer_cliente.NOMBRE_CIUDAD.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO BLANCO
                                // GUIA COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[10] = "Empty".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // ALMACEN RECEPTOR
                                // Almacén
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[11] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[12] = "SERVIENTREGA".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // REGIONAL BASE CLIENTES
                                // CANAL
                                if let Some(valor) = fila.get(2).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.REGIONAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = limpiar_cadenaV2(primer_cliente.REGIONAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // PIA BASE CLIENTES
                                // COD.CLIENTE
                                if let Some(valor) = fila.get(2).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CVE);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = limpiar_cadenaV2(primer_cliente.CVE.to_string().as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // OBSERVACION
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[15] = "1".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CANAL BASE CLIENTES
                                // CATEGORÍA

                                if let Some(valor) = fila.get(2).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CANAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = limpiar_cadenaV2(primer_cliente.CANAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO BLANCO
                                // ARTICULO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[17] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO FIJO
                                // ALMACEN
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[18] = "30".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO BLANCO
                                // PEDIDO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[19] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                //FIN

                                println!("FIN");
                            }

                            // Mostrar la matriz después de la modificación
                            println!("MATRIZ CONSOLIDADO:");
                            imprimir_matriz(&matrizMaster);

                            //=====================GUARDAR LOS DATOS CONSOLIDADOS EN LA DB

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "insert into MC_WEB_CONSOLIDADO_CARGA_PEDIDOS (FECHA, PEDIDO_TRASLADO, CENTRO_SUMINISTRADOR, ALMACEN_EMISOR, MATERIAL,
                                              DESCRIPCION_MATERIAL, CANTIDAD, CENTRO, DESCRIPCION_CENTRO, CIUDAD,
                                              GUIA_COURIER, ALMACEN_RECEPTOR, COURIER, CANAL, COD_CLIENTE,
                                              OBSERVACION, CATEGORIA, ARTICULO, ALMACEN, PEDIDO)
                        values (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12, @p13, @p14, @p15, @p16, @p17, @p18, @p19, @p20);";

                            for fila in &matrizMaster {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_cadenaV2(valor)).collect();


                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .bind(&valores_limpios[11])
                                    .bind(&valores_limpios[12])
                                    .bind(&valores_limpios[13])
                                    .bind(&valores_limpios[14])
                                    .bind(&valores_limpios[15])
                                    .bind(&valores_limpios[16])
                                    .bind(&valores_limpios[17])
                                    .bind(&valores_limpios[18])
                                    .bind(&valores_limpios[19])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }

                            return Ok(HttpResponse::Ok().json("¡Archivo válido!"));
                        } else {
                            println!("No todos los elementos son true - Validar datos del archivo.");
                            return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                        }


                        //Fin.
                    } else {
                        println!("No se encontró la hoja 'PEDIDOS_PUNTUALES'");

                        return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                    }

                    // Asegurarse de que el archivo temporal se elimine cuando ya no se necesite
                }
                _ => (),
            }
        }
    }
    Ok(HttpResponse::Ok().json("Archivo procesado exitosamente")) // Puedes ajustar el mensaje según sea necesario
}


//PEDIDO REABASTECIMIENTO
#[post("/pedido_reabastecimiento")]
async fn cargar_validar_file_pedidos_reabastecimiento(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();


        if let Some(name) = content_type.get_name() {
            match name {
                "fileCNT" => {
                    let filename = content_type.get_filename().unwrap();

                    println!("Nombre de alchivo CNT: {}", filename);

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


                    //if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_REABASTECIMIENTO") {
                    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {

                        //Validación todas las columnas
                        let mut boolean_validacion_all: Vec<bool> = vec![];

                        //Número de filas
                        let mut totalFilas: i32 = 0;
                        println!("1ra Columna: Fecha documento");
                        //Validación primera columna
                        let mut boolean_validacion_individual_1: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if let Some(cell) = row.get(0) {
                                // Verificar si la celda contiene datos -- Muy importante
                                if !cell.is_empty() {
                                    println!("{:?}", cell);
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
                        println!("Validación individual: {:?}", boolean_validacion_individual_1);

                        if boolean_validacion_individual_1.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_1.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("2ra Columna: Pedido SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_2: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(1) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_2.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_2);

                        if boolean_validacion_individual_2.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_2.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("3ra Columna: Centro Suministrador");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(2) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //Número de filas
                        println!("4ta Columna: ALMACEN EMISOR");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(3) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("5ta Columna: CODIGO SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(4) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //Número de filas
                        println!("6ta Columna: DESCRIPCION");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(5) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("7ma Columna: Cantidad");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(6) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("8va Columna: Por entrg.");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(7) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("9na Columna: BODEGA SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(8) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("10ma Columna: Descripción Centro");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(9) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //Número de filas
                        println!("11va Columna: Almacén RECEPTOR");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }
                            if let Some(cell) = row.get(10) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //12va Columna
                        // ***TODO LLENO

                        let numero_de_items = boolean_validacion_all.len();
                        println!("El número de items en el vector es: {}", numero_de_items);

                        let todos_true: bool = boolean_validacion_all.iter().all(|&x| x == true);

                        if todos_true {
                            println!("¡Todos los elementos son true!");


                            //Convertimos el excel en matriz para poder manipular todos los datos.

                            let mut matriz: Vec<Vec<Option<String>>> = Vec::new();

                            //Saltar dos filas recorrer hasta el total de filas anterior.
                            for (row_index, row) in range.rows().skip(1).take(totalFilas as usize).enumerate() {
                                let mut fila: Vec<Option<String>> = Vec::new();
                                //Número de columnas de la matriz
                                for col_index in 0..=10 {
                                    if let Some(cell) = row.get(col_index) {
                                        let valor_como_string = format!("{:?}", cell);
                                        println!("{}", valor_como_string);
                                        fila.push(Some(valor_como_string));
                                    } else {
                                        fila.push(None);
                                    }
                                }

                                // Agregar la fila a la matriz
                                matriz.push(fila);

                                // Añadir una 13va columna con valor por defecto 1000
                                // if let Some(fila) = matriz.last_mut() {
                                //     fila.push(Some("1000".to_string()));
                                // }

                                // ... Resto del código ...
                            }

                            println!("MATRIZ PEDIDOS REABASTECIMIENTO:");
                            println!("{:?}", matriz);

                            println!("Imprimir Valores Matriz:");
                            //Imprimir la matriz con valores limpios
                            for fila in &matriz {
                                for cell in fila {
                                    if let Some(valor) = cell {
                                        // Limpiar el formato Float("valor") y String("valor")
                                        let valor_limpio = valor
                                            .replace("DateTime(", "")
                                            .replace("Float(", "")
                                            .replace("String(\"", "")
                                            .replace("\")", "")
                                            .replace(".0)", "");

                                        // Imprimir el valor numérico o limpio
                                        if let Ok(valor_numerico) = valor_limpio.parse::<f64>() {
                                            print!("{} ", valor_numerico);
                                        } else {
                                            print!("{} ", valor_limpio);
                                        }
                                    } else {
                                        print!("Empty ");
                                    }
                                }
                                println!();
                            }


                            //===================GUARDAMOS LOS DATOS EN LA DB===================

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "INSERT INTO MC_WEB_PEDIDOS_REABASTECIMIENTO (FECHA_DOCUMENTO, PEDIDO_SAP, CENTRO_SUMINISTRADOR, ALMACEN_EMISOR,
                                             CODIGO_SAP, DESCRIPCION, CANTIDAD, POR_ENTREGAR, BODEGA_SAP,
                                             DESCRIPCION_CENTRO, ALMACEN_RECEPTOP) VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11);";

                            for fila in &matriz {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_valor(valor)).collect();

                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }


                            //Imprimir solo los valores de la columna uno
                            // for fila in &matriz {
                            //     if let Some(valor) = fila.get(0).and_then(|v| v.as_ref()) {
                            //         print!("{} ", valor);
                            //     } else {
                            //         print!("Empty ");
                            //     }
                            //     println!();
                            // }


                            //============== SEGUNDO BLOQUE CÓDIGO=============================

                            // Crear una matriz de 5x3 inicializada con "Empty"

                            //El numero 5 es una variable
                            let mut matrizMaster: Vec<Vec<String>> = vec![vec![String::from("Empty"); 20]; totalFilas as usize];

                            // Mostrar la matriz antes de la modificación
                            println!("Matriz original:");
                            imprimir_matriz(&matrizMaster);

                            // Obtener la fecha actual
                            let fecha_actual = Local::now();

                            // Formatear la fecha como "18/10/2023"
                            let fecha_formateada = fecha_actual.format("%d/%m/%Y").to_string();


                            for fila in &matriz {

                                // FECHA ACTUAL - FECHA
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[0] = fecha_formateada.to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //PEDIDO SAP - Pedido Traslado
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    //print!("{} ", valor);

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[1] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CAMPO FIJO - Centro Suministrador
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[2] = "CD06 CLD Telef Móvil Celistics".to_string();  // Asignar valores de la primera columna como cadenas
                                }


                                // ALMACEN EMISOR - Almacén emisor
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[3] = "1000".to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //CODIGO SAP - Material
                                if let Some(valor) = fila.get(4).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[4] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }


                                //DESCRIPCION - Descripción Material
                                if let Some(valor) = fila.get(5).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[5] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CANTIDAD - Cantidad
                                if let Some(valor) = fila.get(6).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[6] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //BODEGA SAP - Centro
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[7] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //SI ES INDIRECTO DEL CAMPO "AGENTE COMERCIAL",CASO CONTRARIO CRUZO BODEGA SAP BASE CLIENTES "DESCRIPCION ALMACEN"
                                // Descripción Centro
                                if let Some(valor) = fila.get(9).and_then(|v| v.as_ref()) {
                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[8] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // COD CIUDAD
                                // CIUDAD
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT T1.*
                                                            FROM WMS_EC.dbo.MC_CLIENTE_CNT T0
                                                            INNER JOIN WMS_EC.dbo.MC_WEB_PROVINCIAS_CIUDADES T1 ON T1.ID_CIUDAD = T0.PROVINCIA
                                                            WHERE T0.CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<MC_WEB_PROVINCIAS_CIUDADES>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.NOMBRE_CIUDAD);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = limpiar_cadenaV2(primer_cliente.NOMBRE_CIUDAD.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO BLANCO
                                // GUIA COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[10] = "Empty".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // ALMACEN RECEPTOR
                                // Almacén
                                if let Some(valor) = fila.get(10).and_then(|v| v.as_ref()) {
                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[11] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[12] = "SERVIENTREGA".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // REGIONAL BASE CLIENTES
                                // CANAL
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.REGIONAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = limpiar_cadenaV2(primer_cliente.REGIONAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // PIA BASE CLIENTES
                                // COD.CLIENTE
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CVE);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = limpiar_cadenaV2(primer_cliente.CVE.to_string().as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // OBSERVACION
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[15] = "1".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CANAL BASE CLIENTES
                                // CATEGORÍA

                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CANAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = limpiar_cadenaV2(primer_cliente.CANAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO BLANCO
                                // ARTICULO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[17] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO FIJO
                                // ALMACEN
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[18] = "30".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO BLANCO
                                // PEDIDO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[19] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                //FIN

                                println!("FIN");
                            }

                            // Mostrar la matriz después de la modificación
                            println!("MATRIZ CONSOLIDADO:");
                            imprimir_matriz(&matrizMaster);

                            //=====================GUARDAR LOS DATOS CONSOLIDADOS EN LA DB

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "insert into MC_WEB_CONSOLIDADO_CARGA_PEDIDOS (FECHA, PEDIDO_TRASLADO, CENTRO_SUMINISTRADOR, ALMACEN_EMISOR, MATERIAL,
                                              DESCRIPCION_MATERIAL, CANTIDAD, CENTRO, DESCRIPCION_CENTRO, CIUDAD,
                                              GUIA_COURIER, ALMACEN_RECEPTOR, COURIER, CANAL, COD_CLIENTE,
                                              OBSERVACION, CATEGORIA, ARTICULO, ALMACEN, PEDIDO)
                        values (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12, @p13, @p14, @p15, @p16, @p17, @p18, @p19, @p20);";

                            for fila in &matrizMaster {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_cadenaV2(valor)).collect();


                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .bind(&valores_limpios[11])
                                    .bind(&valores_limpios[12])
                                    .bind(&valores_limpios[13])
                                    .bind(&valores_limpios[14])
                                    .bind(&valores_limpios[15])
                                    .bind(&valores_limpios[16])
                                    .bind(&valores_limpios[17])
                                    .bind(&valores_limpios[18])
                                    .bind(&valores_limpios[19])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }

                            return Ok(HttpResponse::Ok().json("¡Archivo válido!"));
                        } else {
                            println!("No todos los elementos son true - Validar datos del archivo.");
                            return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                        }


                        //Fin.
                    } else {
                        println!("No se encontró la hoja 'PEDIDOS_PUNTUALES'");

                        return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                    }

                    // Asegurarse de que el archivo temporal se elimine cuando ya no se necesite
                }
                _ => (),
            }
        }
    }
    Ok(HttpResponse::Ok().json("Archivo procesado exitosamente")) // Puedes ajustar el mensaje según sea necesario
}


#[post("/pedido_pop")]
async fn cargar_validar_file_pedidos_pop(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field.content_disposition();


        if let Some(name) = content_type.get_name() {
            match name {
                "fileCNT" => {
                    let filename = content_type.get_filename().unwrap();

                    println!("Nombre de alchivo CNT: {}", filename);

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


                    //if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_REABASTECIMIENTO") {
                    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {


                        //Validación todas las columnas
                        let mut boolean_validacion_all: Vec<bool> = vec![];

                        //Número de filas
                        let mut totalFilas: i32 = 0;
                        println!("1ra Columna: CODIGO PIA");
                        //Validación primera columna
                        let mut boolean_validacion_individual_1: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if let Some(cell) = row.get(0) {
                                // Verificar si la celda contiene datos -- Muy importante
                                if !cell.is_empty() {
                                    println!("{:?}", cell);
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
                        println!("Validación individual: {:?}", boolean_validacion_individual_1);

                        if boolean_validacion_individual_1.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_1.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("2ra Columna: CODIGO SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_2: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(1) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_2.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_2);

                        if boolean_validacion_individual_2.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_2.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("3ra Columna: DESCRIPCION");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(2) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //Número de filas
                        println!("4ta Columna: ESTADO");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(3) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("5ta Columna: CANTIDAD");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(4) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }

                        //Número de filas
                        println!("6ta Columna: Bodega open");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(5) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("7ma Columna: Bodega SAP");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(6) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("8va Columna: Descripción");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(7) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }


                        //Número de filas
                        println!("9na Columna: ALMACEN");
                        //Validación primera columna
                        let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                        for (row_index, row) in range.rows().skip(1).enumerate() {
                            if row_index >= totalFilas as usize {
                                break;  // Termina el bucle después de las primeras 4 iteraciones ejem
                            }

                            if let Some(cell) = row.get(8) {
                                println!("{:?}", cell);
                                //Saber el número de filas.
                                boolean_validacion_individual_3.push(true);
                            }
                        }

                        println!("Validación individual: {:?}", boolean_validacion_individual_3);

                        if boolean_validacion_individual_3.len() == totalFilas as usize {

                            // Verificar si todos los elementos son true
                            let todos_true = boolean_validacion_individual_3.iter().all(|&x| x == true);

                            if todos_true {
                                println!("Todos los elementos son true.");
                                boolean_validacion_all.push(true);
                            } else {
                                println!("No todos los elementos son true.");
                            }
                        }



                        //12va Columna
                        // ***TODO LLENO

                        let numero_de_items = boolean_validacion_all.len();
                        println!("El número de items en el vector es: {}", numero_de_items);

                        let todos_true: bool = boolean_validacion_all.iter().all(|&x| x == true);

                        if todos_true {
                            println!("¡Todos los elementos son true!");

                            //Convertimos el excel en matriz para poder manipular todos los datos.

                            let mut matriz: Vec<Vec<Option<String>>> = Vec::new();

                            //Saltar dos filas recorrer hasta el total de filas anterior.
                            for (row_index, row) in range.rows().skip(1).take(totalFilas as usize).enumerate() {
                                let mut fila: Vec<Option<String>> = Vec::new();
                                //Número de columnas de la matriz
                                for col_index in 0..=8 {
                                    if let Some(cell) = row.get(col_index) {
                                        let valor_como_string = format!("{:?}", cell);
                                        println!("{}", valor_como_string);
                                        fila.push(Some(valor_como_string));
                                    } else {
                                        fila.push(None);
                                    }
                                }

                                // Agregar la fila a la matriz
                                matriz.push(fila);

                                // Añadir una 13va columna con valor por defecto 1000
                                // if let Some(fila) = matriz.last_mut() {
                                //     fila.push(Some("1000".to_string()));
                                // }

                                // ... Resto del código ...
                            }

                            println!("MATRIZ PEDIDOS REABASTECIMIENTO:");
                            println!("{:?}", matriz);

                            println!("Imprimir Valores Matriz:");
                            //Imprimir la matriz con valores limpios
                            for fila in &matriz {
                                for cell in fila {
                                    if let Some(valor) = cell {
                                        // Limpiar el formato Float("valor") y String("valor")
                                        let valor_limpio = valor
                                            .replace("DateTime(", "")
                                            .replace("Float(", "")
                                            .replace("String(\"", "")
                                            .replace("\")", "")
                                            .replace(".0)", "");

                                        // Imprimir el valor numérico o limpio
                                        if let Ok(valor_numerico) = valor_limpio.parse::<f64>() {
                                            print!("{} ", valor_numerico);
                                        } else {
                                            print!("{} ", valor_limpio);
                                        }
                                    } else {
                                        print!("Empty ");
                                    }
                                }
                                println!();
                            }


                            //===================GUARDAMOS LOS DATOS EN LA DB===================

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "INSERT INTO MC_WEB_PEDIDOS_POP (CODIGO_PIA, CODIGO_SAP, DESCRIPCION, ESTADO, CANTIDAD, BODEGA_OPEN, BODEGA_SAP,
                                DESCRIPCION_ALMACEN, ALMACEN) VALUES (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9);";

                            for fila in &matriz {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_valor(valor)).collect();

                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }


                            //Imprimir solo los valores de la columna uno
                            // for fila in &matriz {
                            //     if let Some(valor) = fila.get(0).and_then(|v| v.as_ref()) {
                            //         print!("{} ", valor);
                            //     } else {
                            //         print!("Empty ");
                            //     }
                            //     println!();
                            // }


                            //============== SEGUNDO BLOQUE CÓDIGO=============================

                            // Crear una matriz de 5x3 inicializada con "Empty"

                            //El numero 5 es una variable
                            let mut matrizMaster: Vec<Vec<String>> = vec![vec![String::from("Empty"); 20]; totalFilas as usize];

                            // Mostrar la matriz antes de la modificación
                            println!("Matriz original:");
                            imprimir_matriz(&matrizMaster);

                            // Obtener la fecha actual
                            let fecha_actual = Local::now();

                            // Formatear la fecha como "18/10/2023"
                            let fecha_formateada = fecha_actual.format("%d/%m/%Y").to_string();


                            for fila in &matriz {

                                // FECHA ACTUAL - FECHA
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[0] = fecha_formateada.to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //PEDIDO SAP - Pedido Traslado
                                if let Some(valor) = fila.get(1).and_then(|v| v.as_ref()) {
                                    //print!("{} ", valor);

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[1] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CAMPO FIJO - Centro Suministrador
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[2] = "CD06 CLD Telef Móvil Celistics".to_string();  // Asignar valores de la primera columna como cadenas
                                }


                                // ALMACEN EMISOR - Almacén emisor
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[3] = "1000".to_string();  // Asignar valores de la primera columna como cadenas
                                }

                                //CODIGO SAP - Material
                                if let Some(valor) = fila.get(4).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[4] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }


                                //DESCRIPCION - Descripción Material
                                if let Some(valor) = fila.get(5).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[5] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //CANTIDAD - Cantidad
                                if let Some(valor) = fila.get(6).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[6] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //BODEGA SAP - Centro
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {

                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[7] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                //SI ES INDIRECTO DEL CAMPO "AGENTE COMERCIAL",CASO CONTRARIO CRUZO BODEGA SAP BASE CLIENTES "DESCRIPCION ALMACEN"
                                // Descripción Centro
                                if let Some(valor) = fila.get(9).and_then(|v| v.as_ref()) {
                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[8] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // COD CIUDAD
                                // CIUDAD
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT T1.*
                                                            FROM WMS_EC.dbo.MC_CLIENTE_CNT T0
                                                            INNER JOIN WMS_EC.dbo.MC_WEB_PROVINCIAS_CIUDADES T1 ON T1.ID_CIUDAD = T0.PROVINCIA
                                                            WHERE T0.CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<MC_WEB_PROVINCIAS_CIUDADES>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.NOMBRE_CIUDAD);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = limpiar_cadenaV2(primer_cliente.NOMBRE_CIUDAD.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[9] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO BLANCO
                                // GUIA COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[10] = "Empty".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // ALMACEN RECEPTOR
                                // Almacén
                                if let Some(valor) = fila.get(10).and_then(|v| v.as_ref()) {
                                    // Añadir valores a la primera columna de matrizMaster
                                    for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                        fila_master[11] = limpiar_cadenaV2(valor).to_string();  // Asignar valores de la primera columna como cadenas
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // COURIER
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[12] = "SERVIENTREGA".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // REGIONAL BASE CLIENTES
                                // CANAL
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.REGIONAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = limpiar_cadenaV2(primer_cliente.REGIONAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[13] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // PIA BASE CLIENTES
                                // COD.CLIENTE
                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CVE);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = limpiar_cadenaV2(primer_cliente.CVE.to_string().as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[14] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO FIJO
                                // OBSERVACION
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[15] = "1".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CANAL BASE CLIENTES
                                // CATEGORÍA

                                if let Some(valor) = fila.get(8).and_then(|v| v.as_ref()) {
                                    let valor_str = limpiar_cadenaV2(valor.as_str());
                                    println!("Valor a consultar en la DB: {}", valor_str);

                                    let mut connection = establish_connection().await.unwrap();

                                    let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", valor_str);

                                    println!("Query: {}", query);

                                    let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                                        .fetch_all(&mut connection)
                                        .await;

                                    match cli {
                                        Ok(clientes) => {

                                            // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                                            if let Some(primer_cliente) = clientes.get(0) {
                                                println!("DESCRIPCION_ALMACEN Cliente: {:?}", primer_cliente.CANAL);

                                                // Añadir valores a la primera columna de matrizMaster
                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = limpiar_cadenaV2(primer_cliente.CANAL.as_str()).to_string();  // Asignar valores de la primera columna como cadenas
                                                }
                                            } else {
                                                println!("La consulta no devolvió ningún cliente");

                                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                    fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            println!("No se encuentra");
                                            for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                                fila_master[16] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                            }
                                        }
                                    }
                                } else {
                                    print!("Empty ");
                                }

                                // CAMPO BLANCO
                                // ARTICULO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[17] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO FIJO
                                // ALMACEN
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[18] = "30".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                // CAMPO BLANCO
                                // PEDIDO
                                for (i, fila_master) in matrizMaster.iter_mut().enumerate() {
                                    fila_master[19] = "Empy".parse()?;  // Asignar valores de la primera columna como cadenas
                                }

                                //FIN

                                println!("FIN");
                            }

                            // Mostrar la matriz después de la modificación
                            println!("MATRIZ CONSOLIDADO:");
                            imprimir_matriz(&matrizMaster);

                            //=====================GUARDAR LOS DATOS CONSOLIDADOS EN LA DB

                            // Establecer la conexión fuera del bucle
                            let mut connection = establish_connection().await.unwrap();

                            // Construir la consulta SQL fuera del bucle
                            let query = "insert into MC_WEB_CONSOLIDADO_CARGA_PEDIDOS (FECHA, PEDIDO_TRASLADO, CENTRO_SUMINISTRADOR, ALMACEN_EMISOR, MATERIAL,
                                              DESCRIPCION_MATERIAL, CANTIDAD, CENTRO, DESCRIPCION_CENTRO, CIUDAD,
                                              GUIA_COURIER, ALMACEN_RECEPTOR, COURIER, CANAL, COD_CLIENTE,
                                              OBSERVACION, CATEGORIA, ARTICULO, ALMACEN, PEDIDO)
                        values (@p1, @p2, @p3, @p4, @p5, @p6, @p7, @p8, @p9, @p10, @p11, @p12, @p13, @p14, @p15, @p16, @p17, @p18, @p19, @p20);";

                            for fila in &matrizMaster {

                                // Limpiar cada valor en la fila
                                let valores_limpios: Vec<String> =
                                    fila.iter().map(|valor| limpiar_cadenaV2(valor)).collect();


                                // Ejecutar la consulta dentro del bucle
                                if let Err(err) = sqlx::query(query)
                                    .bind(&valores_limpios[0])
                                    .bind(&valores_limpios[1])
                                    .bind(&valores_limpios[2])
                                    .bind(&valores_limpios[3])
                                    .bind(&valores_limpios[4])
                                    .bind(&valores_limpios[5])
                                    .bind(&valores_limpios[6])
                                    .bind(&valores_limpios[7])
                                    .bind(&valores_limpios[8])
                                    .bind(&valores_limpios[9])
                                    .bind(&valores_limpios[10])
                                    .bind(&valores_limpios[11])
                                    .bind(&valores_limpios[12])
                                    .bind(&valores_limpios[13])
                                    .bind(&valores_limpios[14])
                                    .bind(&valores_limpios[15])
                                    .bind(&valores_limpios[16])
                                    .bind(&valores_limpios[17])
                                    .bind(&valores_limpios[18])
                                    .bind(&valores_limpios[19])
                                    .execute(&mut connection)
                                    .await
                                {
                                    eprintln!("Error al insertar en la base de datos: {}", err);
                                    // Puedes optar por devolver un Result a la función o manejar el error de otra manera.
                                }
                            }

                            return Ok(HttpResponse::Ok().json("¡Archivo válido!"));
                        } else {
                            println!("No todos los elementos son true - Validar datos del archivo.");
                            return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                        }


                        //Fin.
                    } else {
                        println!("No se encontró la hoja 'PEDIDOS_PUNTUALES'");

                        return Ok(HttpResponse::BadRequest().json("Formato de archivo incorrecto"));
                    }

                    // Asegurarse de que el archivo temporal se elimine cuando ya no se necesite
                }
                _ => (),
            }
        }
    }
    Ok(HttpResponse::Ok().json("Archivo procesado exitosamente")) // Puedes ajustar el mensaje según sea necesario
}



pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/plantilla")
        .service(cargar_validar_file_pedidos_puntuales)
        .service(cargar_validar_file_pedidos_indirectos)
        .service(cargar_validar_file_pedidos_reabastecimiento)
        .service(cargar_validar_file_pedidos_pop)
        .service(reporte_consolidado)
        .service(update_pedido_consolidado)
        ;

    conf.service(scope);
}


