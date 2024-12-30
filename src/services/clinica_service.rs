use crate::models::{Clinica, Cliente};
use crate::repositories::clinica_repository::ClinicaRepository;
use uuid::Uuid;

pub struct ClinicaService<T: ClinicaRepository> {
    repository: T,
}

impl<T: ClinicaRepository> ClinicaService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn crear_clinica(
        &mut self,
        nombre: String,
        direccion: String,
        telefono: String,
        correo: String,
    ) -> Result<Clinica, String> {
        let clinica = Clinica::new(nombre, direccion, telefono, correo);
        self.repository.guardar(clinica.clone())?;
        Ok(clinica)
    }

    pub fn obtener_clinica(&self, id: Uuid) -> Option<&Clinica> {
        self.repository.obtener(id)
    }

    pub fn listar_clinicas(&self) -> Vec<&Clinica> {
        self.repository.listar()
    }

    pub fn obtener_clientes_clinica(&self, id_clinica: Uuid) -> Vec<&Cliente> {
        self.repository.obtener_clientes(id_clinica)
    }

    pub fn agregar_cliente(
        &mut self,
        id_clinica: Uuid,
        cliente: Cliente,
    ) -> Result<(), String> {
        // Verificamos que la clínica exista
        if self.repository.obtener(id_clinica).is_none() {
            return Err("La clínica no existe".to_string());
        }
        
        self.repository.agregar_cliente(id_clinica, cliente)
    }
}
