use actix_web::{web, App, HttpServer, Responder, post, get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct ClientPortafolio {
    id_cliente: String,
    monto: f64,
    ano: i32,
    mes: i32,
}

#[derive(Deserialize)]
struct LoanRequest {
    monto: f64,
}

#[derive(Serialize, Clone)]
struct LoanResponse {
    aprobado: bool,
    monto_aprobado: f64,
    tabla_pago: Vec<f64>,
}

#[derive(Serialize)]
struct LoanStatus {
    estado: String,
    monto: Option<f64>,
}

struct AppState {
    max_loan: f64,
    loan_status: Option<LoanResponse>,
}

#[post("/portafolio")]
async fn upload_portafolio(
    data: web::Data<Arc<Mutex<AppState>>>,
    csv_file: String,
) -> impl Responder {
    use std::collections::HashMap;

    let mut rdr = csv::Reader::from_reader(csv_file.as_bytes());
    let mut portafolio: Vec<ClientPortafolio> = Vec::new();

    for result in rdr.deserialize() {
        let record: ClientPortafolio = match result {
            Ok(rec) => rec,
            Err(_) => {
                eprintln!("Error procesando una fila del CSV");
                continue;
            },
        };
        portafolio.push(record);
    }

    // Agrupar ingresos por mes y año
    let mut monthly_revenue: HashMap<(i32, i32), f64> = HashMap::new();
    for record in &portafolio {
        let key = (record.ano, record.mes);
        *monthly_revenue.entry(key).or_insert(0.0) += record.monto;
    }

    // Calcular ingresos totales y meses activos
    let total_revenue: f64 = monthly_revenue.values().sum();
    let months_active = monthly_revenue.len() as f64;
    let mrr = if months_active > 0.0 {
        total_revenue / months_active
    } else {
        0.0
    };

    // Definir churn rate y calcular score
    let churn_rate = 10.0; 
    let churn_rate_porc = churn_rate/100.0;
    let score = (mrr / 100.0) - (churn_rate_porc * 10.0);

    // Calcular el máximo adelanto basado en el score
    let max_loan = if score > 70.0 { mrr } else { 0.0 };

    // Depuración
    println!("MRR: {:.2}", mrr);
    println!("Churn Rate: {:.2}", churn_rate);
    println!("Score: {:.2}", score);
    println!("Max Loan: {:.2}", max_loan);

    // Guardar el máximo préstamo en el estado compartido
    let mut state = data.lock().unwrap();
    state.max_loan = max_loan;

    format!("Máximo adelanto calculado: {:.2}", max_loan)
}


#[post("/apply-loan")]
async fn apply_loan(
    data: web::Data<Arc<Mutex<AppState>>>,
    loan_request: web::Json<LoanRequest>,
) -> impl Responder {
    // Obtener el estado compartido
    let mut state = data.lock().unwrap();

    // Verificar si el máximo préstamo ha sido calculado
    if state.max_loan == 0.0 {
        return web::Json(LoanResponse {
            aprobado: false,
            monto_aprobado: 0.0,
            tabla_pago: vec![],
        });
    }

    // Validar el monto solicitado
    if loan_request.monto > state.max_loan {
        return web::Json(LoanResponse {
            aprobado: false,
            monto_aprobado: 0.0,
            tabla_pago: vec![],
        });
    }

    // Generar la tabla de pagos (distribuida en 12 meses)
    let monthly_payment = loan_request.monto / 12.0;
    let tabla_pago: Vec<f64> = vec![monthly_payment; 12];

    // Crear respuesta del préstamo aprobado
    let loan_response = LoanResponse {
        aprobado: true,
        monto_aprobado: loan_request.monto,
        tabla_pago: tabla_pago.clone(),
    };

    // Actualizar el estado del préstamo en el estado compartido
    state.loan_status = Some(loan_response.clone());

    // Retornar la respuesta
    web::Json(loan_response)
}

#[get("/loan-status")]
async fn loan_status(data: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let state = data.lock().unwrap();

    match &state.loan_status {
        Some(status) => web::Json(LoanStatus {
            estado: "Aprobado".to_string(),
            monto: Some(status.monto_aprobado),
        }),
        None => web::Json(LoanStatus {
            estado: "Pendiente".to_string(),
            monto: None,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(Arc::new(Mutex::new(AppState {
        max_loan: 0.0,
        loan_status: None,
    })));

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(upload_portafolio)
            .service(apply_loan)
            .service(loan_status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
