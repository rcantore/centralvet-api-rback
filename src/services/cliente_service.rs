use crate::models::Cliente;
use crate::repositories::cliente_repository::ClienteRepository;
use uuid::Uuid;

pub struct ClienteService<T: ClienteRepository> {
    repository: T,
}

impl<T: ClienteRepository> ClienteService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn crear_cliente(
        &mut self,
        nombre: String,
        apellido: String,
        correo: String,
        telefono: String,
        direccion: String,
        id_clinica: Uuid,
    ) -> Result<Cliente, String> {
        let cliente = Cliente::new(nombre, apellido, correo, telefono, direccion, id_clinica);
        self.repository.guardar(cliente.clone())?;
        Ok(cliente)
    }

    pub fn obtener_cliente(&self, id: Uuid) -> Option<&Cliente> {
        self.repository.obtener(id)
    }

    pub fn listar_clientes(&self) -> Vec<&Cliente> {
        self.repository.listar()
    }

    pub fn actualizar_cliente(
        &mut self,
        id: Uuid,
        nombre: String,
        apellido: String,
        correo: String,
        telefono: String,
        direccion: String,
        id_clinica: Uuid,
    ) -> Result<Cliente, String> {
        let cliente = self.repository.obtener(id)
            .ok_or_else(|| "El cliente no existe".to_string())?;

        let cliente_actualizado = Cliente {
            id: cliente.id,
            nombre,
            apellido,
            correo,
            telefono,
            direccion,
            id_clinica,
        };

        self.repository.guardar(cliente_actualizado.clone())?;
        Ok(cliente_actualizado)
    }
}
