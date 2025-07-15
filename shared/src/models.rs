use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adherent {
    pub nom: String,
    pub prenom: String,
    pub date_naissance: NaiveDate,
    pub email: String,
    // autres champs selon HelloAsso/Kalisport
    pub deja_exporte: bool,
}
