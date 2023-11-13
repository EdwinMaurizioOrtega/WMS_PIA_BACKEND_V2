use actix_web::{get, Responder, web};
use calamine::{open_workbook, Reader};
use crate::database::connection::establish_connection;
use crate::models::mc_cliente_cnt::McClienteCntAux;

#[get("/pedidos_puntuales")]
async fn formato_carga_pedido() -> impl Responder {

    // conectar_a_base_de_datos().await;

// Ruta al archivo .xlsx
    let file_path = "/Users/lidenar/Desktop/mc/FORMATOS carga de pedidos.xlsx";

    // Intentar abrir el archivo
    match open_workbook::<calamine::Xlsx<_>, _>(file_path) {
        Ok(mut workbook) => {
            // Obtener la primera hoja del libro de trabajo

            //Validación todas las columnas
            let mut boolean_validacion_all: Vec<bool> = vec![];

            //=============================================
            //PEDIDOS_PUNTUALES

            if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_PUNTUALES") {


                //Número de filas
                let mut totalFilas: i32 = 0;
                println!("1ra Columna: N° BODEGA OPEN");
                //Validación primera columna
                let mut boolean_validacion_individual_uno: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(2).enumerate() {
                    if let Some(cell) = row.get(0) {
                        println!("{:?}", cell);
                        //Saber el número de filas.
                        totalFilas += 1;
                        boolean_validacion_individual_uno.push(true);
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

                //Convertimos el excel en matriz para poder manipular todos los datos.

                let mut matriz: Vec<Vec<Option<String>>> = Vec::new();

                for (row_index, row) in range.rows().skip(2).enumerate() {
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

                //println!("{}", matriz);

                // Imprimir la matriz con valores limpios
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

                // Imprimir solo los valores de la columna uno
                for fila in &matriz {
                    if let Some(valor) = fila.get(0).and_then(|v| v.as_ref()) {
                        print!("{} ", valor);
                    } else {
                        print!("Empty ");
                    }
                    println!();
                }

                //Fin.
            } else {
                println!("No se encontró la hoja 'PEDIDOS_PUNTUALES'");
            }

















            println!("HOJA: PEDIDOS_INDIRECTOS");

            //Validación todas las columnas
            let mut boolean_validacion_all: Vec<bool> = vec![];

            if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_INDIRECTOS") {

                //Número de filas
                let mut totalFilas: i32 = 0;
                println!("1ra Columna: CENTRO");
                //Validación primera columna
                let mut boolean_validacion_individual_1: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
                    if let Some(cell) = row.get(0) {
                        println!("{:?}", cell);
                        //Saber el número de filas.
                        totalFilas += 1;
                        boolean_validacion_individual_1.push(true);
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
                println!("2ra Columna: ALMACEN RECEPTOR");
                //Validación primera columna
                let mut boolean_validacion_individual_2: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                println!("3ra Columna: BODEGA SAP");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                    }
                }

                //Número de filas
                println!("4ta Columna: AGENTE COMERCIAL");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {

                    if let Some(cell) = row.get(3) {

                        println!("{:?}", cell);

                        //Número de bodega SAP, la obtenemos del índice columna 2
                        // let Some(aux_bodega_sap) = row.get(2);
                        //
                        // let mut connection = establish_connection().await.unwrap();
                        //
                        // let query = format!("SELECT * FROM WMS_EC.dbo.MC_CLIENTE_CNT WHERE CL_SAP LIKE '{}'", aux_bodega_sap);
                        //
                        // let cli: Result<Vec<McClienteCntAux>, sqlx::Error> = sqlx::query_as(&query)
                        //     .fetch_all(&mut connection)
                        //     .await;
                        //
                        // match cli {
                        //
                        //     Ok(clientes) => {
                        //
                        //         // Haz algo con los resultados (en este caso, imprimir el estado del primer cliente)
                        //         if let Some(primer_cliente) = clientes.get(0) {
                        //
                        //             println!("DESCRIPCION_ALMACEN: {:?}", primer_cliente.DESCRIPCION_ALMACEN);
                        //             boolean_validacion_individual_3.push(true);
                        //
                        //         } else {
                        //             println!("La consulta no devolvió ningún cliente");
                        //             boolean_validacion_individual_3.push(false);
                        //         }
                        //     }
                        //
                        //     Err(err) => {
                        //         println!("No se encuentra");
                        //         //HttpResponse::InternalServerError().body(err.to_string()),
                        //         boolean_validacion_individual_3.push(false);
                        //     }
                        // }















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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                println!("7ma Columna: CANTIDAD");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                println!("8va Columna: PEDIDO SAP");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                println!("9na Columna: ICC");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                println!("10ma Columna: NO APLICA");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                println!("11va Columna: FACTURA");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
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


                //Número de filas
                println!("12va Columna: OBSERVACIONES");
                //Validación primera columna
                let mut boolean_validacion_individual_3: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(5).enumerate() {
                    if let Some(cell) = row.get(11) {
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

                //13va Columna
                // ALMACEN EMISOR
                // 1000
                // CAMPO FIJO














            }

            //==============================================================================


            println!("HOJA: PEDIDOS_REABASTECIMIENTO");

            //Validación todas las columnas
            let mut boolean_validacion_all: Vec<bool> = vec![];

            if let Some(Ok(range)) = workbook.worksheet_range("PEDIDOS_REABASTECIMIENTO") {

                //Número de filas
                let mut totalFilas: i32 = 0;
                println!("1ra Columna: Fecha documento");
                //Validación primera columna
                let mut boolean_validacion_individual_1: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(2).enumerate() {
                    if let Some(cell) = row.get(0) {
                        println!("{:?}", cell);
                        //Saber el número de filas.
                        totalFilas += 1;
                        boolean_validacion_individual_1.push(true);
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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














            }

            //==============================================================================

            println!("HOJA: POP");

            //Validación todas las columnas
            let mut boolean_validacion_all: Vec<bool> = vec![];

            if let Some(Ok(range)) = workbook.worksheet_range("POP") {

                //Número de filas
                let mut totalFilas: i32 = 0;
                println!("1ra Columna: CODIGO PIA");
                //Validación primera columna
                let mut boolean_validacion_individual_1: Vec<bool> = vec![];
                for (row_index, row) in range.rows().skip(2).enumerate() {
                    if let Some(cell) = row.get(0) {
                        println!("{:?}", cell);
                        //Saber el número de filas.
                        totalFilas += 1;
                        boolean_validacion_individual_1.push(true);
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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
                for (row_index, row) in range.rows().skip(5).enumerate() {
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

            }

            //==============================================================================



            //RESULTADO FINAL

            println!("Validación total: {:?}", boolean_validacion_all);

            if boolean_validacion_all.len() == 11 {

                // Verificar si todos los elementos son true
                let todos_true = boolean_validacion_all.iter().all(|&x| x == true);

                if todos_true {
                    println!("Plantilla válida Nro 1.");
                } else {
                    println!("No válida la plantilla Nro 2.");
                }
            }


            //=============================================
            //PEDIDOS_INDIRECTOS





        }
        Err(e) => {
            println!("Error al abrir el archivo: {:?}", e);
        }
    }

    String::from("Consulta completada") // Puedes ajustar esta respuesta según tus necesidades
}





pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api/plantilla")
        .service(formato_carga_pedido);

    conf.service(scope);
}
