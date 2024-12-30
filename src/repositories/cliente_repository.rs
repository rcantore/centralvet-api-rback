use crate::models::Cliente;
use std::collections::HashMap;
use uuid::Uuid;

pub trait ClienteRepository {
    fn obtener(&self, id: Uuid) -> Option<&Cliente>;
    fn listar(&self) -> Vec<&Cliente>;
    fn guardar(&mut self, cliente: Cliente) -> Result<(), String>;
    fn eliminar(&mut self, id: Uuid) -> Result<(), String>;
}

pub struct InMemoryClienteRepository {
    clientes: HashMap<Uuid, Cliente>,
}

impl InMemoryClienteRepository {
    pub fn new() -> Self {
        Self {
            clientes: HashMap::new(),
        }
    }
}

impl ClienteRepository for InMemoryClienteRepository {
    fn obtener(&self, id: Uuid) -> Option<&Cliente> {
        self.clientes.get(&id)
    }

    fn listar(&self) -> Vec<&Cliente> {
        self.clientes.values().collect()
    }

    fn guardar(&mut self, cliente: Cliente) -> Result<(), String> {
        self.clientes.insert(cliente.id, cliente);
        Ok(())
    }

    fn eliminar(&mut self, id: Uuid) -> Result<(), String> {
        self.clientes.remove(&id);
        Ok(())
    }
}
