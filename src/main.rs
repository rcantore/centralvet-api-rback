#[macro_use] extern crate rocket;

mod models;
mod services;
mod controllers;
mod repositories;

use controllers::{
    clinica_controller::*,
    cliente_controller::*,
    mascota_controller::*,
    historia_clinica_controller::*,
};

use repositories::{
    clinica_repository::InMemoryClinicaRepository,
    cliente_repository::InMemoryClienteRepository,
    mascota_repository::InMemoryMascotaRepository,
    historia_clinica_repository::InMemoryHistoriaClinicaRepository
};
#[cfg(feature = "storage-file")]
use repositories::clinica_repository::FileClinicaRepository;
use services::{
    ClinicaService,
    ClienteService,
    MascotaService,
    HistoriaClinicaService,
};

use std::sync::Mutex;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use log::info;
use env_logger;

fn make_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Permite todos los orígenes
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
        ].into_iter().map(From::from).collect(), // Métodos HTTP permitidos
        allowed_headers: AllowedHeaders::all(), // Permite todos los headers
        allow_credentials: true, // Permite credenciales
        ..Default::default()
    }
    .to_cors()
    .expect("Error configurando CORS")
}

#[launch]
fn rocket() -> _ {
    env_logger::init(); // Inicializa el logger
    // Inicializar servicios
    #[cfg(feature = "storage-memory")]
    let clinica_repository = InMemoryClinicaRepository::new();
    #[cfg(feature = "storage-file")]
    let clinica_repository = FileClinicaRepository::new();
    let cliente_repository = InMemoryClienteRepository::new();
    let mascota_repository = InMemoryMascotaRepository::new();
    let historia_clinica_repository = InMemoryHistoriaClinicaRepository::new();

    let clinica_service = ClinicaService::new(clinica_repository);
    let cliente_service = ClienteService::new(cliente_repository);
    let mascota_service = MascotaService::new(mascota_repository);
    let historia_clinica_service = HistoriaClinicaService::new(historia_clinica_repository);

    rocket::build()
        .attach(make_cors()) // Agregamos el middleware CORS
        .manage(Mutex::new(clinica_service))
        .manage(Mutex::new(cliente_service))
        .manage(Mutex::new(mascota_service))
        .manage(Mutex::new(historia_clinica_service))
        .mount("/api", routes![
            // Clínicas
            listar_clinicas,
            obtener_clinica,
            crear_clinica,
            actualizar_clinica,
            listar_clientes_clinica,
            // Clientes
            listar_clientes,
            obtener_cliente,
            crear_cliente,
            // Mascotas
            listar_mascotas,
            listar_mascotas_cliente,
            obtener_mascota,
            crear_mascota,
            // Historias Clínicas
            obtener_historia_mascota,
            obtener_historia,
            crear_historia,
            listar_entradas,
            crear_entrada,
            actualizar_cliente,
            actualizar_mascota,
        ])
}
