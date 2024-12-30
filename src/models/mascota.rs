use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mascota {
    pub id: Uuid,
    pub nombre: String,
    pub especie: String,
    pub raza: String,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub id_cliente: Uuid,
}

impl Mascota {
    pub fn new(
        nombre: String,
        especie: String,
        raza: String,
        fecha_nacimiento: Option<NaiveDate>,
        id_cliente: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre,
            especie,
            raza,
            fecha_nacimiento,
            id_cliente,
        }
    }
}
