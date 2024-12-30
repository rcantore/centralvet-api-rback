use crate::models::{HistoriaClinica, EntradaHistoriaClinica};
use crate::repositories::historia_clinica_repository::HistoriaClinicaRepository;
use uuid::Uuid;
use chrono::Utc;

pub struct HistoriaClinicaService<T: HistoriaClinicaRepository> {
    repository: T,
}

impl<T: HistoriaClinicaRepository> HistoriaClinicaService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn crear_historia(
        &mut self,
        id_mascota: Uuid,
        id_cliente: Uuid,
    ) -> Result<HistoriaClinica, String> {
        let historia = HistoriaClinica::new(id_mascota, id_cliente);
        self.repository.guardar(historia.clone())?;
        Ok(historia)
    }

    pub fn obtener_historia(&self, id: Uuid) -> Option<&HistoriaClinica> {
        self.repository.obtener(id)
    }

    pub fn obtener_historia_mascota(&self, id_mascota: Uuid) -> Option<&HistoriaClinica> {
        self.repository.obtener_por_mascota(id_mascota)
    }

    pub fn agregar_entrada(
        &mut self,
        id_historia: Uuid,
        descripcion: String,
        diagnostico: String,
        tratamiento: String,
        notas: Option<String>,
    ) -> Result<EntradaHistoriaClinica, String> {
        // Verificar que la historia existe
        if self.repository.obtener(id_historia).is_none() {
            return Err("La historia clínica no existe".to_string());
        }

        let entrada = EntradaHistoriaClinica::new(
            id_historia,
            descripcion,
            diagnostico,
            tratamiento,
            notas,
        );

        self.repository.agregar_entrada(entrada.clone())?;
        Ok(entrada)
    }

    pub fn obtener_entradas(&self, id_historia: Uuid) -> Vec<&EntradaHistoriaClinica> {
        self.repository.obtener_entradas(id_historia)
    }
}
