use crate::types::city::{City, RoadTypes};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CITIES: HashMap<&'static str, City> = HashMap::from([
        (
            "RAG",
            City::new(
                "Ragusa",
                &[("MDR", 33, RoadTypes::Asphalt), ("MOD", 11, RoadTypes::Asphalt), ("COM", 8, RoadTypes::Cobblestone), ("GIA", 17, RoadTypes::Gravel)],
                true
            )
        ),
        ("COM", City::new("Comiso", &[("RAG", 8, RoadTypes::Cobblestone), ("VIT", 8, RoadTypes::Cobblestone)], false)),
        (
            "VIT",
            City::new("Vittoria", &[("COM", 8, RoadTypes::Asphalt), ("GLA", 15, RoadTypes::Asphalt)], false)
        ),
        (
            "MDR",
            City::new("Marina di Ragusa", &[("RAG", 33, RoadTypes::Asphalt), ("POZ", 31, RoadTypes::Gravel)], false)
        ),
        (
            "POZ",
            City::new(
                "Pozzallo",
                &[("MDR", 31, RoadTypes::Gravel), ("MOD", 30, RoadTypes::Asphalt), ("NTO", 41, RoadTypes::Asphalt), ("CAP", 40, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "MOD",
            City::new(
                "Modica",
                &[("RAG", 11, RoadTypes::Asphalt), ("POZ", 30, RoadTypes::Asphalt), ("NTO", 41, RoadTypes::Asphalt), ("CAP", 40, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "CAP",
            City::new(
                "Capo Passero",
                &[("POZ", 40, RoadTypes::Asphalt), ("NTO", 36, RoadTypes::Asphalt), ("MOD", 40, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "NTO",
            City::new(
                "Noto Marioa",
                &[("SIR", 31, RoadTypes::Asphalt), ("CAP", 36, RoadTypes::Asphalt), ("MOD", 41, RoadTypes::Asphalt), ("POZ", 41, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "SIR",
            City::new("Siracusa", &[("NTO", 31, RoadTypes::Asphalt), ("FLO", 14, RoadTypes::Cobblestone), ("AUG", 21, RoadTypes::Asphalt)], true)
        ),
        (
            "AUG",
            City::new("Augusta", &[("SIR", 21, RoadTypes::Asphalt), ("LEN", 24, RoadTypes::Asphalt)], false)
        ),
        (
            "LEN",
            City::new("Lentini", &[("CAT", 31, RoadTypes::Asphalt), ("AUG", 24, RoadTypes::Asphalt)], false)
        ),
        (
            "FLO",
            City::new("Floridia", &[("SIR", 14, RoadTypes::Cobblestone), ("PAL", 18, RoadTypes::Cobblestone)], false)
        ),
        (
            "PAL",
            City::new(
                "Palazzolo Acreide",
                &[("GIA", 12, RoadTypes::Gravel), ("FLO", 18, RoadTypes::Cobblestone), ("CAL", 48, RoadTypes::Cobblestone)],
                false
            )
        ),
        (
            "GIA",
            City::new("Giarratana", &[("RAG", 17, RoadTypes::Gravel), ("PAL", 12, RoadTypes::Gravel)], false)
        ),
        (
            "GLA",
            City::new(
                "Gela",
                &[("VIT", 15, RoadTypes::Asphalt), ("CAL", 37, RoadTypes::Cobblestone), ("ENN", 76, RoadTypes::Cobblestone), ("LIC", 28, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "CAL",
            City::new(
                "Caltagirone",
                &[("GLA", 37, RoadTypes::Asphalt), ("GER", 48, RoadTypes::Gravel), ("PAL", 48, RoadTypes::Cobblestone), ("ENN", 50, RoadTypes::Cobblestone)],
                false
            )
        ),
        (
            "GER",
            City::new("Gerbini", &[("PAT", 10, RoadTypes::Gravel), ("CAL", 48, RoadTypes::Gravel)], false)
        ),
        (
            "PAT",
            City::new("Paterno", &[("CAT", 11, RoadTypes::Cobblestone), ("ADR", 9, RoadTypes::Cobblestone), ("GER", 10, RoadTypes::Gravel)], false)
        ),
        (
            "CAT",
            City::new(
                "Catania",
                &[("LEN", 31, RoadTypes::Asphalt), ("ACI", 18, RoadTypes::Asphalt), ("PAT", 11, RoadTypes::Asphalt), ("MES", 92, RoadTypes::Highway)],
                true
            )
        ),
        (
            "ACI",
            City::new("Acireale", &[("CAT", 18, RoadTypes::Asphalt), ("RIP", 15, RoadTypes::Asphalt)], false)
        ),
        (
            "RIP",
            City::new("Riposte", &[("TAM", 12, RoadTypes::Asphalt), ("ACI", 15, RoadTypes::Asphalt)], false)
        ),
        (
            "TAM",
            City::new("Tambrina", &[("RIP", 12, RoadTypes::Asphalt), ("MES", 37, RoadTypes::Asphalt)], false)
        ),
        (
            "MES",
            City::new("Messina", &[("TAM", 37, RoadTypes::Asphalt), ("MIL", 16, RoadTypes::Asphalt), ("CAT", 92, RoadTypes::Highway), ("RCA", 13, RoadTypes::Ferry)], true)
        ),
        (
            "SOL",
            City::new("Solunto", &[("TMI", 12, RoadTypes::Asphalt), ("PMO", 16, RoadTypes::Asphalt)], false)
        ),
        (
            "TMI",
            City::new(
                "Termini Imerese",
                &[("SOL", 12, RoadTypes::Asphalt), ("MIL", 170, RoadTypes::Highway), ("CEF", 54, RoadTypes::Asphalt), ("PET", 61, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "CEF",
            City::new(
                "Cefalu",
                &[("PET", 60, RoadTypes::Asphalt), ("CAS", 7, RoadTypes::Cobblestone), ("TMI", 54, RoadTypes::Asphalt), ("CAR", 39, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "CAS",
            City::new("Castelbuono", &[("PET", 8, RoadTypes::Cobblestone), ("CEF", 7, RoadTypes::Cobblestone)], false)
        ),
        (
            "PET",
            City::new(
                "Petralia Sottana",
                &[("CEF", 60, RoadTypes::Asphalt), ("CAS", 8, RoadTypes::Cobblestone), ("TMI", 61, RoadTypes::Asphalt), ("NIC", 18, RoadTypes::Gravel)],
                false
            )
        ),
        (
            "NIC",
            City::new("Nicosia", &[("PET", 18, RoadTypes::Gravel), ("ENN", 29, RoadTypes::Gravel)], false)
        ),
        (
            "CAR",
            City::new("Caronia", &[("CEF", 39, RoadTypes::Asphalt), ("PTI", 22, RoadTypes::Asphalt)], false)
        ),
        (
            "PTI",
            City::new("Patti", &[("CAR", 22, RoadTypes::Asphalt), ("CRL", 18, RoadTypes::Asphalt), ("BAR", 17, RoadTypes::Asphalt)], false)
        ),
        (
            "BAR",
            City::new(
                "Barcelona Pozo di Goto",
                &[("MIL", 19, RoadTypes::Asphalt), ("CRL", 14, RoadTypes::Asphalt), ("PTI", 17, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "MIL",
            City::new("Milazzo", &[("BAR", 19, RoadTypes::Asphalt), ("MES", 16, RoadTypes::Asphalt), ("TMI", 170, RoadTypes::Highway)], false)
        ),
        (
            "CRL",
            City::new(
                "Castoreale",
                &[("RAN", 33, RoadTypes::Gravel), ("PTI", 18, RoadTypes::Asphalt), ("BAR", 14, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "RAN",
            City::new("Randanzzo", &[("ADR", 48, RoadTypes::Gravel), ("CRL", 33, RoadTypes::Gravel)], false)
        ),
        (
            "ENN",
            City::new(
                "Enna",
                &[
                    ("ADR", 28, RoadTypes::Cobblestone),
                    ("CAL", 50, RoadTypes::Cobblestone),
                    ("GLA", 76, RoadTypes::Cobblestone),
                    ("CTN", 24, RoadTypes::Asphalt),
                    ("NIC", 25, RoadTypes::Gravel)
                ],
                true
            )
        ),
        (
            "ADR",
            City::new("Adrano", &[("PAT", 9, RoadTypes::Cobblestone), ("RAN", 48, RoadTypes::Gravel), ("ENN", 28, RoadTypes::Cobblestone)], false)
        ),
        (
            "LIC",
            City::new(
                "Licata",
                &[("GLA", 34, RoadTypes::Asphalt), ("CTN", 54, RoadTypes::Asphalt), ("CAN", 34, RoadTypes::Asphalt), ("AGR", 54, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "CTN",
            City::new(
                "Caltanissetta",
                &[("ENN", 24, RoadTypes::Asphalt), ("LIC", 54, RoadTypes::Asphalt), ("CAN", 34, RoadTypes::Asphalt), ("MIS", 93, RoadTypes::Cobblestone)],
                true
            )
        ),
        (
            "CAN",
            City::new("Canicatti", &[("AGR", 28, RoadTypes::Asphalt), ("CTN", 34, RoadTypes::Asphalt), ("LIC", 34, RoadTypes::Asphalt)], false)
        ),
        (
            "AGR",
            City::new(
                "Agrigento",
                &[("CAN", 28, RoadTypes::Asphalt), ("LIC", 54, RoadTypes::Asphalt), ("POR", 14, RoadTypes::Asphalt), ("RIB", 36, RoadTypes::Cobblestone)],
                true
            )
        ),
        (
            "POR",
            City::new("Porto Empedocle", &[("AGR", 14, RoadTypes::Asphalt), ("SCI", 35, RoadTypes::Asphalt)], false)
        ),
        (
            "SCI",
            City::new("Sciacca", &[("MEN", 23, RoadTypes::Asphalt), ("POR", 35, RoadTypes::Asphalt)], false)
        ),
        (
            "RIB",
            City::new("Ribera", &[("AGR", 36, RoadTypes::Cobblestone), ("COR", 39, RoadTypes::Gravel)], false)
        ),
        (
            "COR",
            City::new("Corleone", &[("MIS", 40, RoadTypes::Cobblestone), ("RIB", 39, RoadTypes::Gravel), ("PAR", 63, RoadTypes::Gravel)], false)
        ),
        (
            "MIS",
            City::new("Misilmeri", &[("CTN", 93, RoadTypes::Cobblestone), ("PMO", 21, RoadTypes::Cobblestone), ("COR", 40, RoadTypes::Cobblestone)], false)
        ),
        (
            "PMO",
            City::new(
                "Palermo",
                &[
                    ("SOL", 16, RoadTypes::Asphalt),
                    ("MIS", 21, RoadTypes::Cobblestone),
                    ("MON", 18, RoadTypes::Asphalt),
                    ("ALC", 46, RoadTypes::Asphalt),
                    ("MND", 14, RoadTypes::Asphalt)
                ],
                true
            )
        ),
        (
            "MON",
            City::new("Monreale", &[("PMO", 18, RoadTypes::Asphalt), ("ALC", 28, RoadTypes::Asphalt), ("MND", 50, RoadTypes::Asphalt)], false)
        ),
        (
            "MND",
            City::new("Mondello", &[("PMO", 14, RoadTypes::Asphalt), ("ALC", 39, RoadTypes::Asphalt), ("MON", 50, RoadTypes::Asphalt)], false)
        ),
        (
            "ALC",
            City::new(
                "Alcamo",
                &[
                    ("PMO", 46, RoadTypes::Asphalt),
                    ("MON", 28, RoadTypes::Asphalt),
                    ("MND", 39, RoadTypes::Asphalt),
                    ("CST", 15, RoadTypes::Asphalt),
                    ("SEG", 19, RoadTypes::Cobblestone),
                    ("SAL", 34, RoadTypes::Cobblestone),
                    ("PAR", 41, RoadTypes::Cobblestone),
                    ("CVR", 46, RoadTypes::Cobblestone),
                    ("CFI", 20, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "PAR",
            City::new(
                "Partanna",
                &[
                    ("COR", 63, RoadTypes::Gravel),
                    ("ALC", 41, RoadTypes::Cobblestone),
                    ("CVR", 30, RoadTypes::Cobblestone),
                    ("SAL", 26, RoadTypes::Cobblestone),
                    ("CFI", 48, RoadTypes::Cobblestone),
                    ("SEG", 48, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "SAL",
            City::new(
                "Salemi",
                &[
                    ("CVR", 33, RoadTypes::Cobblestone),
                    ("PAR", 26, RoadTypes::Cobblestone),
                    ("ALC", 34, RoadTypes::Cobblestone),
                    ("CFI", 42, RoadTypes::Cobblestone),
                    ("SEG", 42, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "CFI",
            City::new(
                "Caltafimi",
                &[
                    ("SEG", 14, RoadTypes::Cobblestone),
                    ("ALC", 20, RoadTypes::Cobblestone),
                    ("SAL", 42, RoadTypes::Cobblestone),
                    ("PAR", 48, RoadTypes::Cobblestone),
                    ("CVR", 55, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "SEG",
            City::new(
                "Segesta",
                &[
                    ("TRA", 25, RoadTypes::Cobblestone),
                    ("CFI", 14, RoadTypes::Cobblestone),
                    ("ALC", 19, RoadTypes::Cobblestone),
                    ("PAR", 48, RoadTypes::Cobblestone),
                    ("SAL", 42, RoadTypes::Cobblestone),
                    ("CVR", 55, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "TRA",
            City::new("Trapani", &[("MRS", 38, RoadTypes::Asphalt), ("SEG", 25, RoadTypes::Cobblestone), ("ERI", 15, RoadTypes::Asphalt)], true)
        ),
        (
            "ERI",
            City::new("Erice", &[("TRA", 15, RoadTypes::Asphalt), ("CSV", 23, RoadTypes::Asphalt), ("CST", 22, RoadTypes::Asphalt)], false)
        ),
        (
            "CSV",
            City::new("Capo San  Vito", &[("ERI", 23, RoadTypes::Asphalt), ("CST", 19, RoadTypes::Asphalt)], false)
        ),
        (
            "CST",
            City::new(
                "Castallammare",
                &[
                    ("ERI", 22, RoadTypes::Asphalt),
                    ("CSV", 19, RoadTypes::Asphalt),
                    ("ALC", 15, RoadTypes::Asphalt),
                ],
                false
            )
        ),
        (
            "MRS",
            City::new("Marsala", &[("TRA", 38, RoadTypes::Asphalt), ("MDV", 19, RoadTypes::Asphalt)], false)
        ),
        (
            "CVR",
            City::new(
                "Castelvetrano",
                &[
                    ("MDV", 28, RoadTypes::Asphalt),
                    ("MAR", 29, RoadTypes::Asphalt),
                    ("MEN", 31, RoadTypes::Asphalt),
                    ("PAR", 30, RoadTypes::Cobblestone),
                    ("SAL", 40, RoadTypes::Cobblestone),
                    ("ALC", 46, RoadTypes::Cobblestone),
                    ("CFI", 55, RoadTypes::Cobblestone),
                    ("SEG", 55, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "MDV",
            City::new(
                "Mazara del Vallo",
                &[("CVR", 28, RoadTypes::Asphalt), ("MRS", 42, RoadTypes::Asphalt), ("MEN", 51, RoadTypes::Asphalt), ("MAR", 42, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "MEN",
            City::new(
                "Menfi",
                &[("SCI", 23, RoadTypes::Asphalt), ("CVR", 31, RoadTypes::Asphalt), ("MDV", 51, RoadTypes::Asphalt), ("MAR", 21, RoadTypes::Asphalt)],
                false
            )
        ),
        (
            "MAR",
            City::new(
                "Marinella",
                &[("MEN", 21, RoadTypes::Asphalt), ("CVR", 29, RoadTypes::Asphalt), ("MDV", 42, RoadTypes::Asphalt), ("SEL", 15, RoadTypes::Gravel)],
                false
            )
        ),
        ("SEL", City::new("Selinunte", &[("MAR", 15, RoadTypes::Gravel)], false)),

        //Calabria starts here

        ("RCA", City::new("Reggio Calabria", &[("MEL", 20, RoadTypes::Asphalt), ("SCL", 14, RoadTypes::Asphalt), ("MES", 13, RoadTypes::Ferry)], true)),
        ("SCL", City::new("Scilla", &[("RCA", 14, RoadTypes::Asphalt), ("PMI", 22, RoadTypes::Asphalt)], false)),
        ("PMI", City::new("Palmi", &[("SCL", 22, RoadTypes::Asphalt), ("DNV", 11, RoadTypes::Gravel), ("GIT", 17, RoadTypes::Asphalt)], false)),
        ("GIT", City::new("Gioia Tauro", &[("PMI", 17, RoadTypes::Asphalt), ("CIT", 19, RoadTypes::Cobblestone), ("TRO", 27, RoadTypes::Asphalt)], false)),
        ("CIT", City::new("Cittanova", &[("GIT", 19, RoadTypes::Cobblestone), ("SID", 14, RoadTypes::Cobblestone), ("DIN", 18, RoadTypes::Gravel)], false)),
        ("SID", City::new("Siderno", &[("CIT", 14, RoadTypes::Cobblestone), ("BOV", 9, RoadTypes::Asphalt), ("CAU", 29, RoadTypes::Asphalt)], false)),
        ("BOV", City::new("Bovalino", &[("SID", 9, RoadTypes::Asphalt), ("AFR", 12, RoadTypes::Asphalt), ("DNV", 21, RoadTypes::Gravel)], false)),
        ("DNF", City::new("Delianuova", &[("PMI", 11, RoadTypes::Gravel), ("BOV", 21, RoadTypes::Gravel)], false)),
        ("AFR", City::new("Africo", &[("BOV", 12, RoadTypes::Asphalt), ("MEL", 43, RoadTypes::Asphalt)], false)),
        ("MEL", City::new("Melito", &[("RCA", 20, RoadTypes::Asphalt), ("AFR", 43, RoadTypes::Asphalt)], false)),
        ("TRO", City::new("Tropea", &[("VVA", 17, RoadTypes::Asphalt), ("GIT", 27, RoadTypes::Asphalt)], false)),
        ("VVA", City::new("Vibo Valentia", &[("TRO", 17, RoadTypes::Asphalt), ("PIZ", 12, RoadTypes::Asphalt), ("DIN", 20, RoadTypes::Gravel), ("SOV", 38, RoadTypes::Cobblestone)], true)),
        ("PIZ", City::new("Pizzo", &[("VVA", 12, RoadTypes::Asphalt), ("CUR", 12, RoadTypes::Asphalt)], false)),
        ("DIN", City::new("Dinami", &[("VVA", 20, RoadTypes::Gravel), ("CIT",18, RoadTypes::Gravel)], false)),
        ("CAU", City::new("Caulonia", &[("SID", 29, RoadTypes::Asphalt), ("SOV", 35, RoadTypes::Asphalt)], false)),
        ("SOV", City::new("Soveranto", &[("CAU", 35, RoadTypes::Asphalt), ("VVA", 38, RoadTypes::Cobblestone), ("CUR", 17, RoadTypes::Asphalt), ("CNZ", 21, RoadTypes::Asphalt)], false)),

        ("CUR", City::new("Curinga", &[("PIZ", 12, RoadTypes::Asphalt), ("SOV", 17, RoadTypes::Asphalt), ("FAL", 19, RoadTypes::Asphalt), ("CNZ", 21, RoadTypes::Asphalt)], false)),
        ("CNZ", City::new("Catanzaro", &[("SOV", 21, RoadTypes::Asphalt), ("CUR", 21, RoadTypes::Asphalt), ("BOT", 20, RoadTypes::Highway), ("CUT", 31, RoadTypes::Asphalt)], true)),
        ("BOT", City::new("Botricello", &[("CNZ", 20, RoadTypes::Highway), ("CRO", 25, RoadTypes::Highway)], false)),
        ("CRO", City::new("Crotone", &[("BOT", 25, RoadTypes::Highway), ("CUT", 13, RoadTypes::Asphalt), ("CMA", 31, RoadTypes::Highway)], true)),
        ("CMA", City::new("Ciro Marina", &[("CRO", 31, RoadTypes::Highway), ("ROS", 41, RoadTypes::Highway), ("STR", 8, RoadTypes::Asphalt)], false)),
        ("ROS", City::new("Rossano", &[("CMA", 41, RoadTypes::Highway), ("LBC", 15, RoadTypes::Asphalt)], false)),
        ("STR", City::new("Strongoli", &[("CMA", 8, RoadTypes::Asphalt), ("LBC", 27, RoadTypes::Cobblestone), ("CUT", 16, RoadTypes::Cobblestone)], false)),
        ("CUT", City::new("Cutro", &[("CRO", 13, RoadTypes::Asphalt), ("CNZ", 31, RoadTypes::Asphalt), ("STR", 16, RoadTypes::Asphalt), ("CTE", 19, RoadTypes::Gravel)], false)),
        ("LBC", City::new("Longobucco", &[("ROS", 15, RoadTypes::Asphalt), ("STR", 27, RoadTypes::Cobblestone), ("ACR", 16, RoadTypes::Gravel)], false)),
        ("CTE", City::new("Cotronei", &[("CUT", 19, RoadTypes::Gravel), ("ROG", 22, RoadTypes::Gravel)], false)),
        ("ROG", City::new("Rogliano", &[("CTE", 22, RoadTypes:: Gravel), ("COS", 17, RoadTypes::Cobblestone)], false)),
        ("ACR", City::new("Acri", &[("LBC", 16, RoadTypes::Gravel), ("COS", 28, RoadTypes::Gravel)], false)),
        ("COS", City::new("Cosenza", &[("GRI", 18, RoadTypes::Asphalt), ("ROG", 17, RoadTypes::Cobblestone), ("ACR", 28, RoadTypes::Gravel)], true)),
        ("GRI", City::new("Grimaldi", &[("COS", 18, RoadTypes::Asphalt), ("FAL", 22, RoadTypes::Asphalt)], false)),
        ("FAL", City::new("Falerna", &[("CUR", 19, RoadTypes::Asphalt), ("GRI", 22, RoadTypes::Asphalt), ("PAO", 47, RoadTypes::Highway)], false)),
        ("PAO", City::new("Paolo", &[("FAL", 47, RoadTypes::Highway), ("SCA", 46, RoadTypes::Highway)], false)),
        ("SCA", City::new("Scalea", &[("PAO", 46, RoadTypes::Highway)], false))
    ]);
}
