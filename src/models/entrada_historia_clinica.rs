use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntradaHistoriaClinica {
    pub id: Uuid,
    pub id_historia_clinica: Uuid,
    pub fecha: DateTime<Utc>,
    pub descripcion: String,
    pub diagnostico: String,
    pub tratamiento: String,
    pub notas: Option<String>,
}

impl EntradaHistoriaClinica {
    pub fn new(
        id_historia_clinica: Uuid,
        descripcion: String,
        diagnostico: String,
        tratamiento: String,
        notas: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            id_historia_clinica,
            fecha: Utc::now(),
            descripcion,
            diagnostico,
            tratamiento,
            notas,
        }
    }
}
