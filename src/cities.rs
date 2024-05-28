use crate::types::city::City;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CITIES: HashMap<&'static str, City> = HashMap::from([
        (
            "RAG",
            City::new(
                "Ragusa",
                &[("MDR", 33), ("MOD", 11), ("COM", 8), ("GIA", 17)],
                true
            )
        ),
        ("COM", City::new("Comiso", &[("RAG", 8), ("VIT", 8)], false)),
        (
            "VIT",
            City::new("Vittoria", &[("COM", 8), ("GLA", 15)], false)
        ),
        (
            "MDR",
            City::new("Marina di Ragusa", &[("RAG", 33), ("POZ", 31)], false)
        ),
        (
            "POZ",
            City::new(
                "Pozzallo",
                &[("MDR", 31), ("MOD", 30), ("NTO", 41), ("CAP", 40)],
                false
            )
        ),
        (
            "MOD",
            City::new(
                "Modica",
                &[("RAG", 11), ("POZ", 30), ("NTO", 41), ("CAP", 40)],
                false
            )
        ),
        (
            "CAP",
            City::new(
                "Capo Passero",
                &[("POZ", 40), ("NTO", 36), ("MOD", 40)],
                false
            )
        ),
        (
            "NTO",
            City::new(
                "Noto Marioa",
                &[("SIR", 31), ("CAP", 36), ("MOD", 41), ("POZ", 41)],
                false
            )
        ),
        (
            "SIR",
            City::new("Siracusa", &[("NTO", 31), ("FLO", 14), ("AUG", 21)], true)
        ),
        (
            "AUG",
            City::new("Augusta", &[("SIR", 21), ("LEN", 24)], false)
        ),
        (
            "LEN",
            City::new("Lentini", &[("CAT", 31), ("AUG", 24)], false)
        ),
        (
            "FLO",
            City::new("Floridia", &[("SIR", 14), ("PAL", 18)], false)
        ),
        (
            "PAL",
            City::new(
                "Palazzolo Acreide",
                &[("GIA", 12), ("FLO", 18), ("CAL", 148)],
                false
            )
        ),
        (
            "GIA",
            City::new("Giarratana", &[("RAG", 17), ("PAL", 12)], false)
        ),
        (
            "GLA",
            City::new(
                "Gela",
                &[("VIT", 15), ("CAL", 37), ("ENN", 76), ("LIC", 28)],
                false
            )
        ),
        (
            "CAL",
            City::new(
                "Caltagirone",
                &[("GLA", 37), ("GER", 48), ("PAL", 48), ("ENN", 50)],
                false
            )
        ),
        (
            "GER",
            City::new("Gerbini", &[("PAT", 10), ("CAL", 48)], false)
        ),
        (
            "PAT",
            City::new("Paterno", &[("CAT", 11), ("ADR", 9), ("GER", 10)], false)
        ),
        (
            "CAT",
            City::new(
                "Catania",
                &[("LEN", 31), ("ACI", 18), ("PAT", 11), ("MES", 92)],
                true
            )
        ),
        (
            "ACI",
            City::new("Acireale", &[("CAT", 18), ("RIP", 15)], false)
        ),
        (
            "RIP",
            City::new("Riposte", &[("TAM", 12), ("ACI", 15)], false)
        ),
        (
            "TAM",
            City::new("Tambrina", &[("RIP", 12), ("MES", 37)], false)
        ),
        (
            "MES",
            City::new("Messina", &[("TAM", 37), ("MIL", 16), ("CAT", 92)], true)
        ),
        (
            "SOL",
            City::new("Solunto", &[("TMI", 12), ("PMO", 16)], false)
        ),
        (
            "TMI",
            City::new(
                "Termini Imerese",
                &[("SOL", 12), ("MIL", 170), ("CEF", 54), ("PET", 61)],
                false
            )
        ),
        (
            "CEF",
            City::new(
                "Cefalu",
                &[("PET", 60), ("CAS", 7), ("TMI", 54), ("CAR", 39)],
                false
            )
        ),
        (
            "CAS",
            City::new("Castelbuono", &[("PET", 8), ("CEF", 7)], false)
        ),
        (
            "PET",
            City::new(
                "Petralia Sottana",
                &[("CEF", 60), ("CAS", 8), ("TMI", 61), ("NIC", 18)],
                false
            )
        ),
        (
            "NIC",
            City::new("Nicosia", &[("PET", 18), ("ENN", 29)], false)
        ),
        (
            "CAR",
            City::new("Caronia", &[("CEF", 39), ("PTI", 22)], false)
        ),
        (
            "PTI",
            City::new("Patti", &[("CAR", 22), ("CRL", 18), ("BAR", 17)], false)
        ),
        (
            "BAR",
            City::new(
                "Barcelona Pozo di Goto",
                &[("MIL", 19), ("CRL", 14), ("PTI", 17)],
                false
            )
        ),
        (
            "MIL",
            City::new("Milazzo", &[("BAR", 19), ("MES", 16), ("TMI", 170)], false)
        ),
        (
            "CRL",
            City::new(
                "Castoreale",
                &[("RAN", 33), ("PTI", 18), ("BAR", 14)],
                false
            )
        ),
        (
            "RAN",
            City::new("Randanzzo", &[("ADR", 48), ("CRL", 33)], false)
        ),
        (
            "ENN",
            City::new(
                "Enna",
                &[
                    ("ADR", 28),
                    ("CAL", 50),
                    ("GLA", 76),
                    ("CTN", 24),
                    ("NIC", 25)
                ],
                true
            )
        ),
        (
            "ADR",
            City::new("Adrano", &[("PAT", 9), ("RAN", 48), ("ENN", 28)], false)
        ),
        (
            "LIC",
            City::new(
                "Licata",
                &[("GLA", 34), ("CTN", 54), ("CAN", 34), ("AGR", 54)],
                false
            )
        ),
        (
            "CTN",
            City::new(
                "Caltanissetta",
                &[("ENN", 24), ("LIC", 54), ("CAN", 34), ("MIS", 93)],
                true
            )
        ),
        (
            "CAN",
            City::new("Canicatti", &[("AGR", 28), ("CTN", 34), ("LIC", 34)], false)
        ),
        (
            "AGR",
            City::new(
                "Agrigento",
                &[("CAN", 28), ("LIC", 54), ("POR", 14), ("RIB", 36)],
                true
            )
        ),
        (
            "POR",
            City::new("Porto Empedocle", &[("AGR", 14), ("SCI", 35)], false)
        ),
        (
            "SCI",
            City::new("Sciacca", &[("MEN", 23), ("POR", 35)], false)
        ),
        (
            "RIB",
            City::new("Ribera", &[("AGR", 36), ("COR", 39)], false)
        ),
        (
            "COR",
            City::new("Corleone", &[("MIS", 40), ("RIB", 39), ("PAR", 63)], false)
        ),
        (
            "MIS",
            City::new("Misilmeri", &[("CTN", 93), ("PMO", 21), ("COR", 40)], false)
        ),
        (
            "PMO",
            City::new(
                "Palermo",
                &[
                    ("SOL", 16),
                    ("MIS", 21),
                    ("MON", 18),
                    ("ALC", 46),
                    ("MND", 14)
                ],
                true
            )
        ),
        (
            "MON",
            City::new("Monreale", &[("PMO", 18), ("ALC", 28), ("MND", 50)], false)
        ),
        (
            "MND",
            City::new("Mondello", &[("PMO", 14), ("ALC", 39), ("MON", 50)], false)
        ),
        (
            "ALC",
            City::new(
                "Alcamo",
                &[
                    ("PMO", 46),
                    ("MON", 28),
                    ("MND", 39),
                    ("CST", 15),
                    ("SEG", 19),
                    ("SAL", 34),
                    ("PAR", 41),
                    ("CVR", 46),
                    ("CFI", 20)
                ],
                false
            )
        ),
        (
            "PAR",
            City::new(
                "Partanna",
                &[
                    ("COR", 63),
                    ("ALC", 41),
                    ("CVR", 30),
                    ("SAL", 26),
                    ("CFI", 48),
                    ("SEG", 48),
                    ("CST", 46)
                ],
                false
            )
        ),
        (
            "SAL",
            City::new(
                "Salemi",
                &[
                    ("CVR", 33),
                    ("PAR", 26),
                    ("ALC", 34),
                    ("CFI", 42),
                    ("SEG", 42),
                    ("CST", 40)
                ],
                false
            )
        ),
        (
            "CFI",
            City::new(
                "Caltafimi",
                &[
                    ("SEG", 14),
                    ("CST", 24),
                    ("ALC", 20),
                    ("SAL", 42),
                    ("PAR", 48),
                    ("CVR", 55)
                ],
                false
            )
        ),
        (
            "SEG",
            City::new(
                "Segesta",
                &[
                    ("TRA", 25),
                    ("CFI", 14),
                    ("ALC", 19),
                    ("CST", 23),
                    ("PAR", 48),
                    ("SAL", 42),
                    ("CVR", 55)
                ],
                false
            )
        ),
        (
            "TRA",
            City::new("Trapani", &[("MRS", 38), ("SEG", 25), ("ERI", 15)], true)
        ),
        (
            "ERI",
            City::new("Erice", &[("TRA", 15), ("CSV", 23), ("CST", 22)], false)
        ),
        (
            "CSV",
            City::new("Capo San  Vito", &[("ERI", 23), ("CST", 19)], false)
        ),
        (
            "CST",
            City::new(
                "Castallammare",
                &[
                    ("ERI", 22),
                    ("CSV", 19),
                    ("SEG", 23),
                    ("CFI", 24),
                    ("ALC", 15),
                    ("SAL", 40),
                    ("PAR", 46),
                    ("CVR", 48)
                ],
                false
            )
        ),
        (
            "MRS",
            City::new("Marsala", &[("TRA", 38), ("MDV", 19)], false)
        ),
        (
            "CVR",
            City::new(
                "Castelvetrano",
                &[
                    ("MDV", 28),
                    ("MAR", 29),
                    ("MEN", 31),
                    ("PAR", 30),
                    ("SAL", 40),
                    ("ALC", 46),
                    ("CFI", 55),
                    ("SEG", 55),
                    ("CST", 48)
                ],
                false
            )
        ),
        (
            "MDV",
            City::new(
                "Mazara del Vallo",
                &[("CVR", 28), ("MRS", 42), ("MEN", 51), ("MAR", 42)],
                false
            )
        ),
        (
            "MEN",
            City::new(
                "Menfi",
                &[("SCI", 23), ("CVR", 31), ("MDV", 51), ("MAR", 21)],
                false
            )
        ),
        (
            "MAR",
            City::new(
                "Marinella",
                &[("MEN", 21), ("CVR", 29), ("MDV", 42), ("SEL", 15)],
                false
            )
        ),
        ("SEL", City::new("Selinunte", &[("MAR", 15)], false))
    ]);
}
