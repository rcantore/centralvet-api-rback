use crate::models::Mascota;
use std::collections::HashMap;
use uuid::Uuid;

pub trait MascotaRepository {
    fn obtener(&self, id: Uuid) -> Option<&Mascota>;
    fn listar_por_cliente(&self, id_cliente: Uuid) -> Vec<&Mascota>;
    fn guardar(&mut self, mascota: Mascota) -> Result<(), String>;
    fn eliminar(&mut self, id: Uuid) -> Result<(), String>;
    fn listar(&self) -> Vec<&Mascota>;
}

pub struct InMemoryMascotaRepository {
    mascotas: HashMap<Uuid, Mascota>,
}

impl InMemoryMascotaRepository {
    pub fn new() -> Self {
        Self {
            mascotas: HashMap::new(),
        }
    }
}

impl MascotaRepository for InMemoryMascotaRepository {
    fn obtener(&self, id: Uuid) -> Option<&Mascota> {
        self.mascotas.get(&id)
    }

    fn listar_por_cliente(&self, id_cliente: Uuid) -> Vec<&Mascota> {
        self.mascotas.values()
            .filter(|m| m.id_cliente == id_cliente)
            .collect()
    }

    fn guardar(&mut self, mascota: Mascota) -> Result<(), String> {
        self.mascotas.insert(mascota.id, mascota);
        Ok(())
    }

    fn eliminar(&mut self, id: Uuid) -> Result<(), String> {
        self.mascotas.remove(&id);
        Ok(())
    }

    fn listar(&self) -> Vec<&Mascota> {
        self.mascotas.values().collect()
    }
} 
