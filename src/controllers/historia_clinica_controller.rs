use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use uuid::Uuid;
use crate::models::{HistoriaClinica, EntradaHistoriaClinica};
use crate::services::HistoriaClinicaService;
use crate::repositories::historia_clinica_repository::InMemoryHistoriaClinicaRepository;
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct HistoriaClinicaCreateDto {
    pub id_mascota: String,
    pub id_cliente: String,
}

#[derive(Debug, Deserialize)]
pub struct EntradaHistoriaClinicaCreateDto {
    pub descripcion: String,
    pub diagnostico: String,
    pub tratamiento: String,
    pub notas: Option<String>,
}

type HistoriaClinicaServiceType = Mutex<HistoriaClinicaService<InMemoryHistoriaClinicaRepository>>;

#[get("/mascotas/<id_mascota>/historia-clinica")]
pub async fn obtener_historia_mascota(
    id_mascota: String,
    service: &State<HistoriaClinicaServiceType>
) -> Result<Json<HistoriaClinica>, Status> {
    let uuid = Uuid::parse_str(&id_mascota).map_err(|_| Status::BadRequest)?;
    
    service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_historia_mascota(uuid)
        .map(|historia| Json(historia.clone()))
        .ok_or(Status::NotFound)
}

#[get("/historias-clinicas/<id>")]
pub async fn obtener_historia(
    id: String,
    service: &State<HistoriaClinicaServiceType>
) -> Result<Json<HistoriaClinica>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_historia(uuid)
        .map(|historia| Json(historia.clone()))
        .ok_or(Status::NotFound)
}

#[post("/historias-clinicas", data = "<historia_dto>")]
pub async fn crear_historia(
    historia_dto: Json<HistoriaClinicaCreateDto>,
    service: &State<HistoriaClinicaServiceType>
) -> Result<Json<HistoriaClinica>, Status> {
    let id_mascota = Uuid::parse_str(&historia_dto.id_mascota)
        .map_err(|_| Status::BadRequest)?;
    let id_cliente = Uuid::parse_str(&historia_dto.id_cliente)
        .map_err(|_| Status::BadRequest)?;

    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .crear_historia(id_mascota, id_cliente);

    match result {
        Ok(historia) => Ok(Json(historia)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/historias-clinicas/<id>/entradas")]
pub async fn listar_entradas(
    id: String,
    service: &State<HistoriaClinicaServiceType>
) -> Result<Json<Vec<EntradaHistoriaClinica>>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    let entradas = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_entradas(uuid)
        .into_iter()
        .cloned()
        .collect();
    
    Ok(Json(entradas))
}

#[post("/historias-clinicas/<id>/entradas", data = "<entrada_dto>")]
pub async fn crear_entrada(
    id: String,
    entrada_dto: Json<EntradaHistoriaClinicaCreateDto>,
    service: &State<HistoriaClinicaServiceType>
) -> Result<Json<EntradaHistoriaClinica>, Status> {
    let id_historia = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;

    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .agregar_entrada(
            id_historia,
            entrada_dto.descripcion.clone(),
            entrada_dto.diagnostico.clone(),
            entrada_dto.tratamiento.clone(),
            entrada_dto.notas.clone(),
        );

    match result {
        Ok(entrada) => Ok(Json(entrada)),
        Err(_) => Err(Status::InternalServerError),
    }
}
