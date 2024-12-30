use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::Cliente;
use crate::services::ClienteService;
use crate::repositories::cliente_repository::InMemoryClienteRepository;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct ClienteCreateDto {
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub telefono: String,
    pub direccion: String,
    pub id_clinica: String,
}

type ClienteServiceType = Mutex<ClienteService<InMemoryClienteRepository>>;

#[get("/clientes")]
pub async fn listar_clientes(service: &State<ClienteServiceType>) -> Result<Json<Vec<Cliente>>, Status> {
    let clientes = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .listar_clientes()
        .into_iter()
        .cloned()
        .collect();
    Ok(Json(clientes))
}

#[get("/clientes/<id>")]
pub async fn obtener_cliente(
    id: String,
    service: &State<ClienteServiceType>
) -> Result<Json<Cliente>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_cliente(uuid)
        .map(|cliente| Json(cliente.clone()))
        .ok_or(Status::NotFound)
}

#[post("/clientes", data = "<cliente_dto>")]
pub async fn crear_cliente(
    cliente_dto: Json<ClienteCreateDto>,
    service: &State<ClienteServiceType>
) -> Result<Json<Cliente>, Status> {
    let id_clinica = Uuid::parse_str(&cliente_dto.id_clinica)
        .map_err(|_| Status::BadRequest)?;

    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .crear_cliente(
            cliente_dto.nombre.clone(),
            cliente_dto.apellido.clone(),
            cliente_dto.correo.clone(),
            cliente_dto.telefono.clone(),
            cliente_dto.direccion.clone(),
            id_clinica,
        );

    match result {
        Ok(cliente) => Ok(Json(cliente)),
        Err(_) => Err(Status::InternalServerError),
    }
}
