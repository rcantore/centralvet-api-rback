use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use serde::Deserialize;
use uuid::Uuid;
use crate::models::{Cliente, Clinica};
use crate::services::ClinicaService;
use crate::repositories::clinica_repository::InMemoryClinicaRepository;
use std::sync::Mutex;

// DTO para crear una clínica
#[derive(Debug, Deserialize)]
pub struct ClinicaCreateDto {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
    pub correo: String,
}

type ClinicaServiceType = Mutex<ClinicaService<InMemoryClinicaRepository>>;

#[get("/clinicas")]
pub async fn listar_clinicas(service: &State<ClinicaServiceType>) -> Result<Json<Vec<Clinica>>, Status> {
    let clinicas = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .listar_clinicas()
        .into_iter()
        .cloned()
        .collect();
    Ok(Json(clinicas))
}

#[get("/clinicas/<id>")]
pub async fn obtener_clinica(
    id: String,
    service: &State<ClinicaServiceType>
) -> Result<Json<Clinica>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_clinica(uuid)
        .map(|clinica| Json(clinica.clone()))
        .ok_or(Status::NotFound)
}

#[post("/clinicas", data = "<clinica_dto>")]
pub async fn crear_clinica(
    clinica_dto: Json<ClinicaCreateDto>,
    service: &State<ClinicaServiceType>
) -> Result<Json<Clinica>, Status> {
    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .crear_clinica(
            clinica_dto.nombre.clone(),
            clinica_dto.direccion.clone(),
            clinica_dto.telefono.clone(),
            clinica_dto.correo.clone(),
        );

    match result {
        Ok(clinica) => Ok(Json(clinica)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/clinicas/<id>/clientes")]
pub async fn listar_clientes_clinica(
    id: String,
    service: &State<ClinicaServiceType>
) -> Result<Json<Vec<Cliente>>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    let clientes = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_clientes_clinica(uuid)
        .into_iter()
        .cloned()
        .collect();
    
    Ok(Json(clientes))
}

#[put("/clinicas/<id>", data = "<clinica_dto>")]
pub async fn actualizar_clinica(
    id: &str,
    clinica_dto: Json<ClinicaCreateDto>,
    service: &State<ClinicaServiceType>
) -> Result<Json<Clinica>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .actualizar_clinica(
            uuid,
            clinica_dto.nombre.clone(),
            clinica_dto.direccion.clone(),
            clinica_dto.telefono.clone(),
            clinica_dto.correo.clone(),
        );

    match result {
        Ok(clinica) => Ok(Json(clinica)),
        Err(_) => Err(Status::NotFound),
    }
}
