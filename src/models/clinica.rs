use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Clinica {
    pub id: Uuid,
    pub nombre: String,
    pub direccion: String,
    pub telefono: String,
    pub correo: String,
}

impl Clinica {
    pub fn new(nombre: String, direccion: String, telefono: String, correo: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre,
            direccion,
            telefono,
            correo,
        }
    }
}
