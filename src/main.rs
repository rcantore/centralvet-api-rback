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
    historia_clinica_repository::InMemoryHistoriaClinicaRepository,
};
use services::{
    ClinicaService,
    ClienteService,
    MascotaService,
    HistoriaClinicaService,
};

use std::sync::Mutex;

#[launch]
fn rocket() -> _ {
    // Inicializar servicios
    let clinica_repository = InMemoryClinicaRepository::new();
    let cliente_repository = InMemoryClienteRepository::new();
    let mascota_repository = InMemoryMascotaRepository::new();
    let historia_clinica_repository = InMemoryHistoriaClinicaRepository::new();

    let clinica_service = ClinicaService::new(clinica_repository);
    let cliente_service = ClienteService::new(cliente_repository);
    let mascota_service = MascotaService::new(mascota_repository);
    let historia_clinica_service = HistoriaClinicaService::new(historia_clinica_repository);

    rocket::build()
        .manage(Mutex::new(clinica_service))
        .manage(Mutex::new(cliente_service))
        .manage(Mutex::new(mascota_service))
        .manage(Mutex::new(historia_clinica_service))
        .mount("/api", routes![
            // Clínicas
            listar_clinicas,
            obtener_clinica,
            crear_clinica,
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
        ])
}
