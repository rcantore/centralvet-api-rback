use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use serde::Deserialize;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::models::Mascota;
use crate::services::MascotaService;
use crate::repositories::mascota_repository::InMemoryMascotaRepository;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct MascotaCreateDto {
    pub nombre: String,
    pub especie: String,
    pub raza: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_cliente: String,
}

type MascotaServiceType = Mutex<MascotaService<InMemoryMascotaRepository>>;

#[get("/mascotas")] 
pub async fn listar_mascotas(
    service: &State<MascotaServiceType>
) -> Result<Json<Vec<Mascota>>, Status> {
    let mascotas = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .listar_mascotas()
        .into_iter()
        .cloned()
        .collect();

    Ok(Json(mascotas))
}

#[get("/mascotas?<id_cliente>")]
pub async fn listar_mascotas_cliente(
    id_cliente: String,
    service: &State<MascotaServiceType>
) -> Result<Json<Vec<Mascota>>, Status> {
    let uuid = Uuid::parse_str(&id_cliente).map_err(|_| Status::BadRequest)?;
    
    let mascotas = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .listar_mascotas_cliente(uuid)
        .into_iter()
        .cloned()
        .collect();
    
    Ok(Json(mascotas))
}

#[get("/mascotas/<id>")]
pub async fn obtener_mascota(
    id: String,
    service: &State<MascotaServiceType>
) -> Result<Json<Mascota>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    
    service.lock()
        .map_err(|_| Status::InternalServerError)?
        .obtener_mascota(uuid)
        .map(|mascota| Json(mascota.clone()))
        .ok_or(Status::NotFound)
}

#[post("/mascotas", data = "<mascota_dto>")]
pub async fn crear_mascota(
    mascota_dto: Json<MascotaCreateDto>,
    service: &State<MascotaServiceType>
) -> Result<Json<Mascota>, Status> {
    let id_cliente = Uuid::parse_str(&mascota_dto.id_cliente)
        .map_err(|_| Status::BadRequest)?;

    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .crear_mascota(
            mascota_dto.nombre.clone(),
            mascota_dto.especie.clone(),
            mascota_dto.raza.clone(),
            mascota_dto.fecha_nacimiento,
            id_cliente,
        );

    match result {
        Ok(mascota) => Ok(Json(mascota)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/mascotas/<id>", data = "<mascota_dto>")]
pub async fn actualizar_mascota(
    id: String,
    mascota_dto: Json<MascotaCreateDto>,
    service: &State<MascotaServiceType>
) -> Result<Json<Mascota>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let id_cliente = Uuid::parse_str(&mascota_dto.id_cliente)
        .map_err(|_| Status::BadRequest)?;

    let result = service.lock()
        .map_err(|_| Status::InternalServerError)?
        .actualizar_mascota(
            uuid,
            mascota_dto.nombre.clone(),
            mascota_dto.especie.clone(),
            mascota_dto.raza.clone(),
            mascota_dto.fecha_nacimiento,
            id_cliente,
        );

    match result {
        Ok(mascota) => Ok(Json(mascota)),
        Err(_) => Err(Status::NotFound),
    }
}
