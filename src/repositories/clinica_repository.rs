use crate::models::{Clinica, Cliente};
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "storage-file")]
use super::file_repository::FileRepository;

pub trait ClinicaRepository {
    fn obtener(&self, id: Uuid) -> Option<&Clinica>;
    fn listar(&self) -> Vec<&Clinica>;
    fn guardar(&mut self, clinica: Clinica) -> Result<(), String>;
    fn eliminar(&mut self, id: Uuid) -> Result<(), String>;
    
    // Operaciones relacionadas con clientes
    fn obtener_clientes(&self, id_clinica: Uuid) -> Vec<&Cliente>;
    fn agregar_cliente(&mut self, id_clinica: Uuid, cliente: Cliente) -> Result<(), String>;
    fn eliminar_cliente(&mut self, id_clinica: Uuid, id_cliente: Uuid) -> Result<(), String>;
}

pub struct InMemoryClinicaRepository {
    clinicas: HashMap<Uuid, Clinica>,
    clientes_por_clinica: HashMap<Uuid, Vec<Cliente>>,
}

impl InMemoryClinicaRepository {
    pub fn new() -> Self {
        Self {
            clinicas: HashMap::new(),
            clientes_por_clinica: HashMap::new(),
        }
    }
}

impl ClinicaRepository for InMemoryClinicaRepository {
    fn obtener(&self, id: Uuid) -> Option<&Clinica> {
        self.clinicas.get(&id)
    }

    fn listar(&self) -> Vec<&Clinica> {
        self.clinicas.values().collect()
    }

    fn guardar(&mut self, clinica: Clinica) -> Result<(), String> {
        self.clinicas.insert(clinica.id, clinica);
        Ok(())
    }

    fn eliminar(&mut self, id: Uuid) -> Result<(), String> {
        self.clinicas.remove(&id);
        Ok(())
    }

    fn obtener_clientes(&self, id_clinica: Uuid) -> Vec<&Cliente> {
        self.clientes_por_clinica.get(&id_clinica)
            .map(|clientes| clientes.iter().collect())
            .unwrap_or_else(|| Vec::new())
    }

    fn agregar_cliente(&mut self, id_clinica: Uuid, cliente: Cliente) -> Result<(), String> {
        self.clientes_por_clinica.entry(id_clinica).or_default().push(cliente);
        Ok(())
    }

    fn eliminar_cliente(&mut self, id_clinica: Uuid, id_cliente: Uuid) -> Result<(), String> {
        self.clientes_por_clinica.entry(id_clinica).or_default().retain(|c| c.id != id_cliente);
        Ok(())
    }
}

#[cfg(feature = "storage-file")]
pub struct FileClinicaRepository {
    storage: FileRepository<Clinica>,
    cached_clinicas: Vec<Clinica>,
    clientes_por_clinica: HashMap<Uuid, Vec<Cliente>>,
}

#[cfg(feature = "storage-file")]
impl FileClinicaRepository {
    pub fn new() -> Self {
        let storage = FileRepository::new("data/clinicas.json");
        let cached_clinicas = storage.load().unwrap_or_default();
        Self {
            storage,
            cached_clinicas,
            clientes_por_clinica: HashMap::new(),
        }
    }
}

#[cfg(feature = "storage-file")]
impl ClinicaRepository for FileClinicaRepository {
    fn listar(&self) -> Vec<&Clinica> {
        self.cached_clinicas.iter().collect()
    }

    fn guardar(&mut self, clinica: Clinica) -> Result<(), String> {
        if let Some(idx) = self.cached_clinicas.iter().position(|c| c.id == clinica.id) {
            self.cached_clinicas[idx] = clinica;
        } else {
            self.cached_clinicas.push(clinica);
        }
        self.storage.save(self.cached_clinicas.clone())
    }

    fn obtener(&self, id: Uuid) -> Option<&Clinica> {
        self.cached_clinicas.iter().find(|c| c.id == id)
    }

    fn eliminar(&mut self, id: Uuid) -> Result<(), String> {
        self.cached_clinicas.retain(|c| c.id != id);
        self.storage.save(self.cached_clinicas.clone())
    }

    fn obtener_clientes(&self, id_clinica: Uuid) -> Vec<&Cliente> {
        self.clientes_por_clinica.get(&id_clinica)
            .map(|clientes| clientes.iter().collect())
            .unwrap_or_default()
    }

    fn agregar_cliente(&mut self, id_clinica: Uuid, cliente: Cliente) -> Result<(), String> {
        if self.cached_clinicas.iter().find(|c| c.id == id_clinica).is_none() {
            return Err("Clínica no encontrada".to_string());
        }
        
        self.clientes_por_clinica.entry(id_clinica).or_default().push(cliente);
        Ok(())
    }

    fn eliminar_cliente(&mut self, id_clinica: Uuid, id_cliente: Uuid) -> Result<(), String> {
        if self.cached_clinicas.iter().find(|c| c.id == id_clinica).is_none() {
            return Err("Clínica no encontrada".to_string());
        }

        self.clientes_por_clinica
            .entry(id_clinica)
            .or_default()
            .retain(|c| c.id != id_cliente);
        
        Ok(())
    }
} 