use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adherent {
    pub nom: String,
    pub prenom: String,
    pub date_naissance: NaiveDate,
    pub email: String,
    // autres champs selon HelloAsso/Kalisport
    pub deja_exporte: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Saison {
    pub id: u32,
    pub nom: String,
}
