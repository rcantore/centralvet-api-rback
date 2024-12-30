use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cliente {
    pub id: Uuid,
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub telefono: String,
    pub direccion: String,
}

impl Cliente {
    pub fn new(
        nombre: String,
        apellido: String,
        correo: String,
        telefono: String,
        direccion: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre,
            apellido,
            correo,
            telefono,
            direccion,
        }
    }
}
