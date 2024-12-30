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
    ) -> Result<Cliente, String> {
        let cliente = Cliente::new(nombre, apellido, correo, telefono, direccion);
        self.repository.guardar(cliente.clone())?;
        Ok(cliente)
    }

    pub fn obtener_cliente(&self, id: Uuid) -> Option<&Cliente> {
        self.repository.obtener(id)
    }

    pub fn listar_clientes(&self) -> Vec<&Cliente> {
        self.repository.listar()
    }
}
