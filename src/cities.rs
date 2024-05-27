use crate::types::city::City;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CITIES: HashMap<&'static str, City> = HashMap::from([("RAG", City::new("Ragusa", &[("MDR", 33),("MOD", 11),("COM", 8),("GIA", 17),], true)),
("COM", City::new("Comiso", &[("RAG", 8),("VIT", 8),], false)),
("VIT", City::new("Vittoria", &[("COM", 8),("GLA", 15),], false)),
("MDR", City::new("Marina di Ragusa", &[("RAG", 33),("POZ", 31),], false)),
("POZ", City::new("Pozzallo", &[("MDR", 31),("MOD", 30),("NTO", 41),("CAP", 40),], false)),
("MOD", City::new("Modica", &[("RAG", 11),("POZ", 30),("NTO", 41),("CAP", 40),], false)),
("CAP", City::new("Capo Passero", &[("POZ", 40),("NTO", 36),("MOD", 40),], false)),
("NTO", City::new("Noto Marioa", &[("SIR", 31),("CAP", 36),("MOD", 41),("POZ", 41),], false)),
("SIR", City::new("Siracusa", &[("NTO", 31),("FLO", 14),("AUG", 21),], true)),
("AUG", City::new("Augusta", &[("SIR", 21),("LEN", 24),], false)),
("LEN", City::new("Lentini", &[("CAT", 31),("AUG", 24),], false)),
("FLO", City::new("Floridia", &[("SIR", 14),("PAL", 18),], false)),
("PAL", City::new("Palazzolo Accreide", &[("GIA", 12),("FLO", 18),("CAL", 48),], false)),
("GIA", City::new("Giarratana", &[("RAG", 17),("PAL", 12),], false)),
    ]);
}