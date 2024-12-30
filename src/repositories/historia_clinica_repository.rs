use crate::models::{HistoriaClinica, EntradaHistoriaClinica};
use std::collections::HashMap;
use uuid::Uuid;

pub trait HistoriaClinicaRepository {
    fn obtener(&self, id: Uuid) -> Option<&HistoriaClinica>;
    fn obtener_por_mascota(&self, id_mascota: Uuid) -> Option<&HistoriaClinica>;
    fn guardar(&mut self, historia: HistoriaClinica) -> Result<(), String>;
    fn eliminar(&mut self, id: Uuid) -> Result<(), String>;
    
    // Métodos para las entradas
    fn agregar_entrada(&mut self, entrada: EntradaHistoriaClinica) -> Result<(), String>;
    fn obtener_entradas(&self, id_historia: Uuid) -> Vec<&EntradaHistoriaClinica>;
}

pub struct InMemoryHistoriaClinicaRepository {
    historias: HashMap<Uuid, HistoriaClinica>,
    entradas: HashMap<Uuid, Vec<EntradaHistoriaClinica>>,
}

impl InMemoryHistoriaClinicaRepository {
    pub fn new() -> Self {
        Self {
            historias: HashMap::new(),
            entradas: HashMap::new(),
        }
    }
} 

impl HistoriaClinicaRepository for InMemoryHistoriaClinicaRepository {
    fn obtener(&self, id: Uuid) -> Option<&HistoriaClinica> {
        self.historias.get(&id)
    }

    fn obtener_por_mascota(&self, id_mascota: Uuid) -> Option<&HistoriaClinica> {
        self.historias.values()
            .find(|h| h.id_mascota == id_mascota)
    }

    fn guardar(&mut self, historia: HistoriaClinica) -> Result<(), String> {
        self.historias.insert(historia.id, historia);
        Ok(())
    }

    fn eliminar(&mut self, id: Uuid) -> Result<(), String> {
        self.historias.remove(&id);
        Ok(())
    }

    fn agregar_entrada(&mut self, entrada: EntradaHistoriaClinica) -> Result<(), String> {
        self.entradas.entry(entrada.id_historia_clinica).or_insert(vec![]).push(entrada);
        Ok(())
    }

    fn obtener_entradas(&self, id_historia: Uuid) -> Vec<&EntradaHistoriaClinica> {
        self.entradas.get(&id_historia)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }
    
}
