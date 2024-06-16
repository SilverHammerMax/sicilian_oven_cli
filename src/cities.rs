use crate::types::city::Region;
use crate::types::city::{City, RoadTypes};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CITIES: HashMap<&'static str, City> = HashMap::from([
        (
            "RAG",
            City::new(
                "Ragusa",
                Region::Sicily,
                &[
                    ("MDR", 33, RoadTypes::Asphalt),
                    ("MOD", 11, RoadTypes::Asphalt),
                    ("COM", 8, RoadTypes::Cobblestone),
                    ("GIA", 17, RoadTypes::Unpaved)
                ],
                true
            )
        ),
        (
            "COM",
            City::new(
                "Comiso",
                Region::Sicily,
                &[
                    ("RAG", 8, RoadTypes::Cobblestone),
                    ("VIT", 8, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "VIT",
            City::new(
                "Vittoria",
                Region::Sicily,
                &[
                    ("COM", 8, RoadTypes::Asphalt),
                    ("GLA", 15, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MDR",
            City::new(
                "Marina di Ragusa",
                Region::Sicily,
                &[
                    ("RAG", 33, RoadTypes::Asphalt),
                    ("POZ", 31, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "POZ",
            City::new(
                "Pozzallo",
                Region::Sicily,
                &[
                    ("MDR", 31, RoadTypes::Unpaved),
                    ("MOD", 30, RoadTypes::Asphalt),
                    ("NTO", 41, RoadTypes::Asphalt),
                    ("CAP", 40, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MOD",
            City::new(
                "Modica",
                Region::Sicily,
                &[
                    ("RAG", 11, RoadTypes::Asphalt),
                    ("POZ", 30, RoadTypes::Asphalt),
                    ("NTO", 41, RoadTypes::Asphalt),
                    ("CAP", 40, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CAP",
            City::new(
                "Capo Passero",
                Region::Sicily,
                &[
                    ("POZ", 40, RoadTypes::Asphalt),
                    ("NTO", 36, RoadTypes::Asphalt),
                    ("MOD", 40, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "NTO",
            City::new(
                "Noto Marioa",
                Region::Sicily,
                &[
                    ("SIR", 31, RoadTypes::Asphalt),
                    ("CAP", 36, RoadTypes::Asphalt),
                    ("MOD", 41, RoadTypes::Asphalt),
                    ("POZ", 41, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "SIR",
            City::new(
                "Siracusa",
                Region::Sicily,
                &[
                    ("NTO", 31, RoadTypes::Asphalt),
                    ("FLO", 14, RoadTypes::Cobblestone),
                    ("AUG", 21, RoadTypes::Asphalt)
                ],
                true
            )
        ),
        (
            "AUG",
            City::new(
                "Augusta",
                Region::Sicily,
                &[
                    ("SIR", 21, RoadTypes::Asphalt),
                    ("LEN", 24, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "LEN",
            City::new(
                "Lentini",
                Region::Sicily,
                &[
                    ("CAT", 31, RoadTypes::Asphalt),
                    ("AUG", 24, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "FLO",
            City::new(
                "Floridia",
                Region::Sicily,
                &[
                    ("SIR", 14, RoadTypes::Cobblestone),
                    ("PAL", 18, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "PAL",
            City::new(
                "Palazzolo Acreide",
                Region::Sicily,
                &[
                    ("GIA", 12, RoadTypes::Unpaved),
                    ("FLO", 18, RoadTypes::Cobblestone),
                    ("CAL", 48, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "GIA",
            City::new(
                "Giarratana",
                Region::Sicily,
                &[
                    ("RAG", 17, RoadTypes::Unpaved),
                    ("PAL", 12, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "GLA",
            City::new(
                "Gela",
                Region::Sicily,
                &[
                    ("VIT", 15, RoadTypes::Asphalt),
                    ("CAL", 37, RoadTypes::Cobblestone),
                    ("ENN", 76, RoadTypes::Cobblestone),
                    ("LIC", 28, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CAL",
            City::new(
                "Caltagirone",
                Region::Sicily,
                &[
                    ("GLA", 37, RoadTypes::Asphalt),
                    ("GER", 48, RoadTypes::Unpaved),
                    ("PAL", 48, RoadTypes::Cobblestone),
                    ("ENN", 50, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "GER",
            City::new(
                "Gerbini",
                Region::Sicily,
                &[
                    ("PAT", 10, RoadTypes::Unpaved),
                    ("CAL", 48, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "PAT",
            City::new(
                "Paterno",
                Region::Sicily,
                &[
                    ("CAT", 11, RoadTypes::Cobblestone),
                    ("ADR", 9, RoadTypes::Cobblestone),
                    ("GER", 10, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "CAT",
            City::new(
                "Catania",
                Region::Sicily,
                &[
                    ("LEN", 31, RoadTypes::Asphalt),
                    ("ACI", 18, RoadTypes::Asphalt),
                    ("PAT", 11, RoadTypes::Asphalt),
                    ("MES", 92, RoadTypes::Highway)
                ],
                true
            )
        ),
        (
            "ACI",
            City::new(
                "Acireale",
                Region::Sicily,
                &[
                    ("CAT", 18, RoadTypes::Asphalt),
                    ("RIP", 15, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "RIP",
            City::new(
                "Riposte",
                Region::Sicily,
                &[
                    ("TAM", 12, RoadTypes::Asphalt),
                    ("ACI", 15, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "TAM",
            City::new(
                "Tambrina",
                Region::Sicily,
                &[
                    ("RIP", 12, RoadTypes::Asphalt),
                    ("MES", 37, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MES",
            City::new(
                "Messina",
                Region::Sicily,
                &[
                    ("TAM", 37, RoadTypes::Asphalt),
                    ("MIL", 16, RoadTypes::Asphalt),
                    ("CAT", 92, RoadTypes::Highway),
                    ("RCA", 13, RoadTypes::Ferry)
                ],
                true
            )
        ),
        (
            "SOL",
            City::new(
                "Solunto",
                Region::Sicily,
                &[
                    ("TMI", 12, RoadTypes::Asphalt),
                    ("PMO", 16, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "TMI",
            City::new(
                "Termini Imerese",
                Region::Sicily,
                &[
                    ("SOL", 12, RoadTypes::Asphalt),
                    ("MIL", 170, RoadTypes::Highway),
                    ("CEF", 54, RoadTypes::Asphalt),
                    ("PET", 61, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CEF",
            City::new(
                "Cefalu",
                Region::Sicily,
                &[
                    ("PET", 60, RoadTypes::Asphalt),
                    ("CAS", 7, RoadTypes::Cobblestone),
                    ("TMI", 54, RoadTypes::Asphalt),
                    ("CAR", 39, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CAS",
            City::new(
                "Castelbuono",
                Region::Sicily,
                &[
                    ("PET", 8, RoadTypes::Cobblestone),
                    ("CEF", 7, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "PET",
            City::new(
                "Petralia Sottana",
                Region::Sicily,
                &[
                    ("CEF", 60, RoadTypes::Asphalt),
                    ("CAS", 8, RoadTypes::Cobblestone),
                    ("TMI", 61, RoadTypes::Asphalt),
                    ("NIC", 18, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "NIC",
            City::new(
                "Nicosia",
                Region::Sicily,
                &[
                    ("PET", 18, RoadTypes::Unpaved),
                    ("ENN", 29, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "CAR",
            City::new(
                "Caronia",
                Region::Sicily,
                &[
                    ("CEF", 39, RoadTypes::Asphalt),
                    ("PTI", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "PTI",
            City::new(
                "Patti",
                Region::Sicily,
                &[
                    ("CAR", 22, RoadTypes::Asphalt),
                    ("CRL", 18, RoadTypes::Asphalt),
                    ("BAR", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "BAR",
            City::new(
                "Barcelona Pozo di Goto",
                Region::Sicily,
                &[
                    ("MIL", 19, RoadTypes::Asphalt),
                    ("CRL", 14, RoadTypes::Asphalt),
                    ("PTI", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MIL",
            City::new(
                "Milazzo",
                Region::Sicily,
                &[
                    ("BAR", 19, RoadTypes::Asphalt),
                    ("MES", 16, RoadTypes::Asphalt),
                    ("TMI", 170, RoadTypes::Highway),
                    ("LIP", 44, RoadTypes::Ferry)
                ],
                false
            )
        ),
        (
            "CRL",
            City::new(
                "Castoreale",
                Region::Sicily,
                &[
                    ("RAN", 33, RoadTypes::Unpaved),
                    ("PTI", 18, RoadTypes::Asphalt),
                    ("BAR", 14, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "RAN",
            City::new(
                "Randanzzo",
                Region::Sicily,
                &[
                    ("ADR", 48, RoadTypes::Unpaved),
                    ("CRL", 33, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "ENN",
            City::new(
                "Enna",
                Region::Sicily,
                &[
                    ("ADR", 28, RoadTypes::Cobblestone),
                    ("CAL", 50, RoadTypes::Cobblestone),
                    ("GLA", 76, RoadTypes::Cobblestone),
                    ("CTN", 24, RoadTypes::Asphalt),
                    ("NIC", 25, RoadTypes::Unpaved)
                ],
                true
            )
        ),
        (
            "ADR",
            City::new(
                "Adrano",
                Region::Sicily,
                &[
                    ("PAT", 9, RoadTypes::Cobblestone),
                    ("RAN", 48, RoadTypes::Unpaved),
                    ("ENN", 28, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "LIC",
            City::new(
                "Licata",
                Region::Sicily,
                &[
                    ("GLA", 34, RoadTypes::Asphalt),
                    ("CTN", 54, RoadTypes::Asphalt),
                    ("CAN", 34, RoadTypes::Asphalt),
                    ("AGR", 54, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CTN",
            City::new(
                "Caltanissetta",
                Region::Sicily,
                &[
                    ("ENN", 24, RoadTypes::Asphalt),
                    ("LIC", 54, RoadTypes::Asphalt),
                    ("CAN", 34, RoadTypes::Asphalt),
                    ("MIS", 93, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "CAN",
            City::new(
                "Canicatti",
                Region::Sicily,
                &[
                    ("AGR", 28, RoadTypes::Asphalt),
                    ("CTN", 34, RoadTypes::Asphalt),
                    ("LIC", 34, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "AGR",
            City::new(
                "Agrigento",
                Region::Sicily,
                &[
                    ("CAN", 28, RoadTypes::Asphalt),
                    ("LIC", 54, RoadTypes::Asphalt),
                    ("POR", 14, RoadTypes::Asphalt),
                    ("RIB", 36, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "POR",
            City::new(
                "Porto Empedocle",
                Region::Sicily,
                &[
                    ("AGR", 14, RoadTypes::Asphalt),
                    ("SCI", 35, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "SCI",
            City::new(
                "Sciacca",
                Region::Sicily,
                &[
                    ("MEN", 23, RoadTypes::Asphalt),
                    ("POR", 35, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "RIB",
            City::new(
                "Ribera",
                Region::Sicily,
                &[
                    ("AGR", 36, RoadTypes::Cobblestone),
                    ("COR", 39, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "COR",
            City::new(
                "Corleone",
                Region::Sicily,
                &[
                    ("MIS", 40, RoadTypes::Cobblestone),
                    ("RIB", 39, RoadTypes::Unpaved),
                    ("PAR", 63, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "MIS",
            City::new(
                "Misilmeri",
                Region::Sicily,
                &[
                    ("CTN", 93, RoadTypes::Cobblestone),
                    ("PMO", 21, RoadTypes::Cobblestone),
                    ("COR", 40, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "PMO",
            City::new(
                "Palermo",
                Region::Sicily,
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
            City::new(
                "Monreale",
                Region::Sicily,
                &[
                    ("PMO", 18, RoadTypes::Asphalt),
                    ("ALC", 28, RoadTypes::Asphalt),
                    ("MND", 50, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MND",
            City::new(
                "Mondello",
                Region::Sicily,
                &[
                    ("PMO", 14, RoadTypes::Asphalt),
                    ("ALC", 39, RoadTypes::Asphalt),
                    ("MON", 50, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "ALC",
            City::new(
                "Alcamo",
                Region::Sicily,
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
                Region::Sicily,
                &[
                    ("COR", 63, RoadTypes::Unpaved),
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
                Region::Sicily,
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
                Region::Sicily,
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
                Region::Sicily,
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
            City::new(
                "Trapani",
                Region::Sicily,
                &[
                    ("MRS", 38, RoadTypes::Asphalt),
                    ("SEG", 25, RoadTypes::Cobblestone),
                    ("ERI", 15, RoadTypes::Asphalt),
                    ("PAN", 110, RoadTypes::Ferry),
                    ("FAV", 22, RoadTypes::Ferry)
                ],
                true
            )
        ),
        (
            "ERI",
            City::new(
                "Erice",
                Region::Sicily,
                &[
                    ("TRA", 15, RoadTypes::Asphalt),
                    ("CSV", 23, RoadTypes::Asphalt),
                    ("CST", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CSV",
            City::new(
                "Capo San  Vito",
                Region::Sicily,
                &[
                    ("ERI", 23, RoadTypes::Asphalt),
                    ("CST", 19, RoadTypes::Asphalt),
                    ("UST", 63, RoadTypes::Ferry)
                ],
                false
            )
        ),
        (
            "CST",
            City::new(
                "Castallammare",
                Region::Sicily,
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
            City::new(
                "Marsala",
                Region::Sicily,
                &[
                    ("TRA", 38, RoadTypes::Asphalt),
                    ("MDV", 19, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CVR",
            City::new(
                "Castelvetrano",
                Region::Sicily,
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
                Region::Sicily,
                &[
                    ("CVR", 28, RoadTypes::Asphalt),
                    ("MRS", 42, RoadTypes::Asphalt),
                    ("MEN", 51, RoadTypes::Asphalt),
                    ("MAR", 42, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MEN",
            City::new(
                "Menfi",
                Region::Sicily,
                &[
                    ("SCI", 23, RoadTypes::Asphalt),
                    ("CVR", 31, RoadTypes::Asphalt),
                    ("MDV", 51, RoadTypes::Asphalt),
                    ("MAR", 21, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MAR",
            City::new(
                "Marinella",
                Region::Sicily,
                &[
                    ("MEN", 21, RoadTypes::Asphalt),
                    ("CVR", 29, RoadTypes::Asphalt),
                    ("MDV", 42, RoadTypes::Asphalt),
                    ("SEL", 15, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "SEL",
            City::new(
                "Selinunte",
                Region::Sicily,
                &[("MAR", 15, RoadTypes::Unpaved)],
                false
            )
        ),
        (
            "FAV",
            City::new(
                "Favignana",
                Region::Sicily,
                &[("TRA", 22, RoadTypes::Ferry)],
                false
            )
        ),
        (
            "TRC",
            City::new(
                "Tracino",
                Region::Sicily,
                &[("PAN", 9, RoadTypes::Cobblestone)],
                false
            )
        ),
        (
            "PAN",
            City::new(
                "Pantelleria",
                Region::Sicily,
                &[
                    ("TRC", 9, RoadTypes::Cobblestone),
                    ("TRA", 110, RoadTypes::Ferry)
                ],
                false
            )
        ),
        (
            "UST",
            City::new(
                "Ustica",
                Region::Sicily,
                &[("CSV", 63, RoadTypes::Ferry), ("MAL", 78, RoadTypes::Ferry)],
                false
            )
        ),
        (
            "MAL",
            City::new(
                "Malfa",
                Region::Sicily,
                &[("UST", 78, RoadTypes::Ferry), ("LIP", 9, RoadTypes::Ferry)],
                false
            )
        ),
        (
            "LIP",
            City::new(
                "Lipari",
                Region::Sicily,
                &[
                    ("MAL", 9, RoadTypes::Ferry),
                    ("SRO", 31, RoadTypes::Ferry),
                    ("MIL", 44, RoadTypes::Ferry)
                ],
                false
            )
        ),
        (
            "SRO",
            City::new(
                "Stromboli",
                Region::Sicily,
                &[
                    ("LIP", 31, RoadTypes::Ferry),
                    ("NAP", 160, RoadTypes::Ferry)
                ],
                false
            )
        ),
        (
            "RCA",
            City::new(
                "Reggio Calabria",
                Region::Calabria,
                &[
                    ("MEL", 20, RoadTypes::Asphalt),
                    ("SCL", 14, RoadTypes::Asphalt),
                    ("MES", 13, RoadTypes::Ferry)
                ],
                true
            )
        ),
        (
            "SCL",
            City::new(
                "Scilla",
                Region::Calabria,
                &[
                    ("RCA", 14, RoadTypes::Asphalt),
                    ("PMI", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "PMI",
            City::new(
                "Palmi",
                Region::Calabria,
                &[
                    ("SCL", 22, RoadTypes::Asphalt),
                    ("DNV", 11, RoadTypes::Unpaved),
                    ("GIT", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "GIT",
            City::new(
                "Gioia Tauro",
                Region::Calabria,
                &[
                    ("PMI", 17, RoadTypes::Asphalt),
                    ("CIT", 19, RoadTypes::Cobblestone),
                    ("TRO", 27, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CIT",
            City::new(
                "Cittanova",
                Region::Calabria,
                &[
                    ("GIT", 19, RoadTypes::Cobblestone),
                    ("SID", 14, RoadTypes::Cobblestone),
                    ("DIN", 18, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "SID",
            City::new(
                "Siderno",
                Region::Calabria,
                &[
                    ("CIT", 14, RoadTypes::Cobblestone),
                    ("BOV", 9, RoadTypes::Asphalt),
                    ("CAU", 29, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "BOV",
            City::new(
                "Bovalino",
                Region::Calabria,
                &[
                    ("SID", 9, RoadTypes::Asphalt),
                    ("AFR", 12, RoadTypes::Asphalt),
                    ("DNV", 21, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "DNV",
            City::new(
                "Delianuova",
                Region::Calabria,
                &[
                    ("PMI", 11, RoadTypes::Unpaved),
                    ("BOV", 21, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "AFR",
            City::new(
                "Africo",
                Region::Calabria,
                &[
                    ("BOV", 12, RoadTypes::Asphalt),
                    ("MEL", 43, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MEL",
            City::new(
                "Melito",
                Region::Calabria,
                &[
                    ("RCA", 20, RoadTypes::Asphalt),
                    ("AFR", 43, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "TRO",
            City::new(
                "Tropea",
                Region::Calabria,
                &[
                    ("VVA", 17, RoadTypes::Asphalt),
                    ("GIT", 27, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "VVA",
            City::new(
                "Vibo Valentia",
                Region::Calabria,
                &[
                    ("TRO", 17, RoadTypes::Asphalt),
                    ("PIZ", 12, RoadTypes::Asphalt),
                    ("DIN", 20, RoadTypes::Unpaved),
                    ("SOV", 38, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "PIZ",
            City::new(
                "Pizzo",
                Region::Calabria,
                &[
                    ("VVA", 12, RoadTypes::Asphalt),
                    ("CUR", 12, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "DIN",
            City::new(
                "Dinami",
                Region::Calabria,
                &[
                    ("VVA", 20, RoadTypes::Unpaved),
                    ("CIT", 18, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "CAU",
            City::new(
                "Caulonia",
                Region::Calabria,
                &[
                    ("SID", 29, RoadTypes::Asphalt),
                    ("SOV", 35, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "SOV",
            City::new(
                "Soveranto",
                Region::Calabria,
                &[
                    ("CAU", 35, RoadTypes::Asphalt),
                    ("VVA", 38, RoadTypes::Cobblestone),
                    ("CUR", 17, RoadTypes::Asphalt),
                    ("CNZ", 21, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CUR",
            City::new(
                "Curinga",
                Region::Calabria,
                &[
                    ("PIZ", 12, RoadTypes::Asphalt),
                    ("SOV", 17, RoadTypes::Asphalt),
                    ("FAL", 19, RoadTypes::Asphalt),
                    ("CNZ", 21, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CNZ",
            City::new(
                "Catanzaro",
                Region::Calabria,
                &[
                    ("SOV", 21, RoadTypes::Asphalt),
                    ("CUR", 21, RoadTypes::Asphalt),
                    ("BOT", 20, RoadTypes::Highway),
                    ("CUT", 31, RoadTypes::Asphalt)
                ],
                true
            )
        ),
        (
            "BOT",
            City::new(
                "Botricello",
                Region::Calabria,
                &[
                    ("CNZ", 20, RoadTypes::Highway),
                    ("CRO", 25, RoadTypes::Highway)
                ],
                false
            )
        ),
        (
            "CRO",
            City::new(
                "Crotone",
                Region::Calabria,
                &[
                    ("BOT", 25, RoadTypes::Highway),
                    ("CUT", 13, RoadTypes::Asphalt),
                    ("CMA", 31, RoadTypes::Highway)
                ],
                true
            )
        ),
        (
            "CMA",
            City::new(
                "Ciro Marina",
                Region::Calabria,
                &[
                    ("CRO", 31, RoadTypes::Highway),
                    ("ROS", 41, RoadTypes::Highway),
                    ("STR", 8, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "ROS",
            City::new(
                "Rossano",
                Region::Calabria,
                &[
                    ("CMA", 41, RoadTypes::Highway),
                    ("LBC", 15, RoadTypes::Asphalt),
                    ("COS", 38, RoadTypes::Cobblestone),
                    ("SIB", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "STR",
            City::new(
                "Strongoli",
                Region::Calabria,
                &[
                    ("CMA", 8, RoadTypes::Asphalt),
                    ("LBC", 27, RoadTypes::Cobblestone),
                    ("CUT", 16, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "CUT",
            City::new(
                "Cutro",
                Region::Calabria,
                &[
                    ("CRO", 13, RoadTypes::Asphalt),
                    ("CNZ", 31, RoadTypes::Asphalt),
                    ("STR", 16, RoadTypes::Asphalt),
                    ("CTE", 19, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "LBC",
            City::new(
                "Longobucco",
                Region::Calabria,
                &[
                    ("ROS", 15, RoadTypes::Asphalt),
                    ("STR", 27, RoadTypes::Cobblestone),
                    ("ACR", 16, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "CTE",
            City::new(
                "Cotronei",
                Region::Calabria,
                &[
                    ("CUT", 19, RoadTypes::Unpaved),
                    ("ROG", 22, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "ROG",
            City::new(
                "Rogliano",
                Region::Calabria,
                &[
                    ("CTE", 22, RoadTypes::Unpaved),
                    ("COS", 17, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "ACR",
            City::new(
                "Acri",
                Region::Calabria,
                &[
                    ("LBC", 16, RoadTypes::Unpaved),
                    ("COS", 28, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "COS",
            City::new(
                "Cosenza",
                Region::Calabria,
                &[
                    ("GRI", 18, RoadTypes::Asphalt),
                    ("ROG", 17, RoadTypes::Cobblestone),
                    ("ACR", 28, RoadTypes::Unpaved),
                    ("PAO", 17, RoadTypes::Unpaved),
                    ("ALT", 32, RoadTypes::Cobblestone),
                    ("ROS", 38, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "GRI",
            City::new(
                "Grimaldi",
                Region::Calabria,
                &[
                    ("COS", 18, RoadTypes::Asphalt),
                    ("FAL", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "FAL",
            City::new(
                "Falerna",
                Region::Calabria,
                &[
                    ("CUR", 19, RoadTypes::Asphalt),
                    ("GRI", 22, RoadTypes::Asphalt),
                    ("PAO", 47, RoadTypes::Highway)
                ],
                false
            )
        ),
        (
            "PAO",
            City::new(
                "Paolo",
                Region::Calabria,
                &[
                    ("FAL", 47, RoadTypes::Highway),
                    ("SCA", 46, RoadTypes::Highway),
                    ("COS", 17, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "SCA",
            City::new(
                "Scalea",
                Region::Calabria,
                &[
                    ("PAO", 46, RoadTypes::Highway),
                    ("MRA", 13, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CVL",
            City::new(
                "Castrovillari",
                Region::Calabria,
                &[
                    ("SCA", 31, RoadTypes::Unpaved),
                    ("ALT", 19, RoadTypes::Unpaved),
                    ("ORI", 26, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "ORI",
            City::new(
                "Oriolo",
                Region::Calabria,
                &[
                    ("RTO", 14, RoadTypes::Cobblestone),
                    ("CVL", 26, RoadTypes::Cobblestone),
                    ("LAU", 30, RoadTypes::Cobblestone),
                    ("TUR", 15, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "RTO",
            City::new(
                "Roseto",
                Region::Calabria,
                &[
                    ("ORI", 14, RoadTypes::Cobblestone),
                    ("SIB", 27, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "SIB",
            City::new(
                "Sibari",
                Region::Calabria,
                &[
                    ("ROS", 17, RoadTypes::Asphalt),
                    ("RTO", 27, RoadTypes::Asphalt),
                    ("ALT", 23, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "ALT",
            City::new(
                "Altomonte",
                Region::Calabria,
                &[
                    ("COS", 32, RoadTypes::Cobblestone),
                    ("SIB", 23, RoadTypes::Unpaved),
                    ("CVL", 19, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "MRA",
            City::new(
                "Maratea",
                Region::Basilicata,
                &[
                    ("SCA", 13, RoadTypes::Asphalt),
                    ("LAU", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
                (
            "LAU",
            City::new(
                "Lauri",
                Region::Basilicata,
                &[
                    ("MRA", 17, RoadTypes::Asphalt),
                    ("TUR", 30, RoadTypes::Cobblestone),
                    ("ORI", 30, RoadTypes::Cobblestone),
                    ("ALI", 26, RoadTypes::Cobblestone),
                    ("BRI", 54, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "ALI",
            City::new(
                "Aliano",
                Region::Basilicata,
                &[
                    ("BRI", 42, RoadTypes::Unpaved),
                    ("FER", 15, RoadTypes::Cobblestone),
                    ("TUR", 16, RoadTypes::Cobblestone),
                    ("LAU", 26, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "FER",
            City::new(
                "Fernandina",
                Region::Basilicata,
                &[
                    ("MAT", 19, RoadTypes::Asphalt),
                    ("POT", 27, RoadTypes::Cobblestone),
                    ("PIS", 16, RoadTypes::Asphalt),
                    ("ALI", 15, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "MAT",
            City::new(
                "Matera",
                Region::Basilicata,
                &[
                    ("POT", 36, RoadTypes::Highway),
                    ("MET", 61, RoadTypes::Asphalt),
                    ("FER", 27, RoadTypes::Asphalt)
                ],
                true
            )
        ),
        (
            "MET",
            City::new(
                "Metaponto",
                Region::Basilicata,
                &[
                    ("MAT", 61, RoadTypes::Asphalt),
                    ("PIS", 11, RoadTypes::Asphalt),
                    ("POL", 21, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "PIS",
            City::new(
                "Pisticci",
                Region::Basilicata,
                &[
                    ("FER", 16, RoadTypes::Asphalt),
                    ("MET", 11, RoadTypes::Asphalt),
                    ("TUR", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "POL",
            City::new(
                "Policoro",
                Region::Basilicata,
                &[
                    ("MET", 21, RoadTypes::Asphalt),
                    ("ROS", 22, RoadTypes::Asphalt),
                    ("TUR", 15, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "POT",
            City::new(
                "Potenza",
                Region::Basilicata,
                &[
                    ("MAT", 36, RoadTypes::Highway),
                    ("FER", 27, RoadTypes::Cobblestone),
                    ("MFI", 25, RoadTypes::Cobblestone),
                    ("BRI", 25, RoadTypes::Unpaved)
                ],
                true
            )
        ),
        (
            "BRI",
            City::new(
                "Brienza",
                Region::Basilicata,
                &[
                    ("POT", 25, RoadTypes::Unpaved),
                    ("MFI", 25, RoadTypes::Unpaved),
                    ("ALI", 42, RoadTypes::Unpaved),
                    ("LAU", 54, RoadTypes::Unpaved),
                    ("POL", 19, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "MFI",
            City::new(
                "Melfi",
                Region::Basilicata,
                &[
                    ("POT", 25, RoadTypes::Cobblestone),
                    ("BRI", 25, RoadTypes::Unpaved),
                    ("LIO", 25, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "TUR",
            City::new(
                "Tursi",
                Region::Basilicata,
                &[
                    ("ORI", 15, RoadTypes::Cobblestone),
                    ("LAU", 30, RoadTypes::Cobblestone),
                    ("POL", 15, RoadTypes::Cobblestone),
                    ("PIS", 17, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "NAP",
            City::new(
                "Napoli",
                Region::Campania,
                &[
                    ("SRN", 31, RoadTypes::Highway),
                    ("NOL", 11, RoadTypes::Asphalt),
                    ("VAR", 17, RoadTypes::Asphalt),
                    ("CTA", 12, RoadTypes::Asphalt),
                    ("SRO", 160, RoadTypes::Ferry),
                    ("ISC", 20, RoadTypes::Ferry)
                ],
                true
            )
        ),
        (
            "ISC",
            City::new(
                "Ischia",
                Region::Campania,
                &[("NAP", 20, RoadTypes::Ferry)],
                false
            )
        ),
        (
            "SAP",
            City::new(
                "Sapri",
                Region::Campania,
                &[
                    ("MRA", 15, RoadTypes::Asphalt),
                    ("CAM", 13, RoadTypes::Asphalt),
                    ("PAD", 25, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "CAM",
            City::new(
                "Camerota",
                Region::Campania,
                &[
                    ("SAP", 13, RoadTypes::Asphalt),
                    ("ASC", 16, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "ASC",
            City::new(
                "Ascea",
                Region::Campania,
                &[
                    ("CAM", 16, RoadTypes::Asphalt),
                    ("AGP", 20, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "PAD",
            City::new(
                "Padula",
                Region::Campania,
                &[
                    ("SAP", 25, RoadTypes::Cobblestone),
                    ("AGP", 26, RoadTypes::Unpaved),
                    ("POA", 16, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "POA",
            City::new(
                "Polla",
                Region::Campania,
                &[
                    ("PAD", 16, RoadTypes::Cobblestone),
                    ("BAT", 24, RoadTypes::Cobblestone),
                    ("CON", 12, RoadTypes::Cobblestone),
                    ("BRI", 19, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "BAT",
            City::new(
                "Battipaglia",
                Region::Campania,
                &[
                    ("SRN", 11, RoadTypes::Asphalt),
                    ("AGP", 21, RoadTypes::Asphalt),
                    ("POL", 24, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "AGP",
            City::new(
                "Agropolis",
                Region::Campania,
                &[
                    ("BAT", 21, RoadTypes::Asphalt),
                    ("PAD", 26, RoadTypes::Unpaved),
                    ("ASC", 20, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "SOR",
            City::new(
                "Sorrento",
                Region::Campania,
                &[
                    ("SRN", 24, RoadTypes::Asphalt),
                ],
                false
            )
        ),
        (
            "SRN",
            City::new(
                "Salerno",
                Region::Campania,
                &[
                    ("NAP", 31, RoadTypes::Highway),
                    ("AVE", 17, RoadTypes::Highway),
                    ("SOR", 24, RoadTypes::Asphalt),
                    ("CON", 23, RoadTypes::Asphalt)
                ],
                true
            )
        ),
        (
            "CON",
            City::new(
                "Contursi",
                Region::Campania,
                &[
                    ("SAL", 23, RoadTypes::Asphalt),
                    ("POL", 12, RoadTypes::Cobblestone),
                    ("LIO", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "NOL",
            City::new(
                "Nola",
                Region::Campania,
                &[
                    ("NAP", 11, RoadTypes::Asphalt),
                    ("AVE", 22, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "AVE",
            City::new(
                "Avellino",
                Region::Campania,
                &[
                    ("SRN", 17, RoadTypes::Highway),
                    ("BEN", 24, RoadTypes::Highway),
                    ("NOL", 22, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "VAR",
            City::new(
                "Varcaturo",
                Region::Campania,
                &[
                    ("NAP", 17, RoadTypes::Asphalt),
                    ("MDG", 27, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "MDG",
            City::new(
                "Mondragone",
                Region::Campania,
                &[
                    ("VAR", 27, RoadTypes::Asphalt),
                    ("TEA", 16, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "TEA",
            City::new(
                "Teano",
                Region::Campania,
                &[
                    ("MDG", 16, RoadTypes::Asphalt),
                    ("CTA", 25, RoadTypes::Asphalt),
                    ("ALF", 15, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "ALF",
            City::new(
                "Alife",
                Region::Campania,
                &[
                    ("BEN", 30, RoadTypes::Asphalt),
                    ("TEA", 15, RoadTypes::Asphalt),
                    ("CTA", 26, RoadTypes::Asphalt),
                    ("CIR", 32, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "CTA",
            City::new(
                "Caserta",
                Region::Campania,
                &[
                    ("NAP", 12, RoadTypes::Asphalt),
                    ("BBA", 31, RoadTypes::Unpaved),
                    ("TEA", 25, RoadTypes::Asphalt),
                    ("ALF", 26, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "BBA",
            City::new(
                "Barba",
                Region::Campania,
                &[
                    ("BEN", 7, RoadTypes::Unpaved),
                    ("CTA", 31, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "LIO",
            City::new(
                "Lioni",
                Region::Campania,
                &[
                    ("BEN", 28, RoadTypes::Asphalt),
                    ("CON", 22, RoadTypes::Asphalt),
                    ("CGO", 10, RoadTypes::Unpaved),
                    ("MFI", 25, RoadTypes::Unpaved)
                ],
                false
            )
        ),
        (
            "BEN",
            City::new(
                "Benevento",
                Region::Campania,
                &[
                    ("AVE", 24, RoadTypes::Highway),
                    ("BBA", 7, RoadTypes::Unpaved),
                    ("LIO", 28, RoadTypes::Asphalt),
                    ("ALF", 30, RoadTypes::Asphalt),
                    ("CIR", 12, RoadTypes::Cobblestone),
                    ("ROM", 12, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "CIR",
            City::new(
                "Circello",
                Region::Campania,
                &[
                    ("BEN", 12, RoadTypes::Cobblestone),
                    ("ROM", 8, RoadTypes::Cobblestone),
                    ("ALF", 32, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "ROM",
            City::new(
                "Romiti",
                Region::Campania,
                &[
                    ("BEN", 12, RoadTypes::Cobblestone),
                    ("CIR", 8, RoadTypes::Cobblestone),
                    ("CGO", 25, RoadTypes::Cobblestone),
                    ("LUC", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CGO",
            City::new(
                "Calaggoi",
                Region::Campania,
                &[
                    ("LIO", 10, RoadTypes::Cobblestone),
                    ("ROM", 25, RoadTypes::Cobblestone),
                    ("CNC", 22, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "LUC",
            City::new(
                "Lucera",
                Region::Apulia,
                &[
                    ("FOG", 20, RoadTypes::Asphalt),
                    ("ROM", 22, RoadTypes::Asphalt),
                    ("TOI", 13, RoadTypes::Cobblestone),
                    ("SSV", 14, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "TOI",
            City::new(
                "Troia",
                Region::Apulia,
                &[
                    ("LUC", 13, RoadTypes::Cobblestone),
                    ("CNC", 17, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "CNC",
            City::new(
                "Catenaccio",
                Region::Apulia,
                &[
                    ("TOI", 17, RoadTypes::Cobblestone),
                    ("CGO", 22, RoadTypes::Asphalt),
                    ("CGN", 19, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "CGN",
            City::new(
                "Cerignola",
                Region::Apulia,
                &[
                    ("FOG", 32, RoadTypes::Cobblestone),
                    ("BRL", 16, RoadTypes::Asphalt),
                    ("AND", 15, RoadTypes::Cobblestone),
                    ("CNC", 19, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "BRL",
            City::new(
                "Barletta",
                Region::Apulia,
                &[
                    ("CGN", 16, RoadTypes::Asphalt),
                    ("TNI", 14, RoadTypes::Asphalt),
                    ("AND", 12, RoadTypes::Asphalt),
                    ("ZAP", 21, RoadTypes::Asphalt)
                ],
                true
            )
        ),
        (
            "FOG",
            City::new(
                "Foggia",
                Region::Apulia,
                &[
                    ("LUC", 20, RoadTypes::Asphalt),
                    ("ZAP", 28, RoadTypes::Asphalt),
                    ("CGN", 32, RoadTypes::Cobblestone),
                    ("APV", 8, RoadTypes::Cobblestone)
                ],
                true
            )
        ),
        (
            "ZAP",
            City::new(
                "Zapponeta",
                Region::Apulia,
                &[
                    ("BRL", 21, RoadTypes::Asphalt),
                    ("FOG", 28, RoadTypes::Asphalt),
                    ("MFR", 19, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "APV",
            City::new(
                "Arpinova",
                Region::Apulia,
                &[
                    ("FOG", 8, RoadTypes::Cobblestone),
                    ("MFR", 17, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "MFR",
            City::new(
                "Manfredonia",
                Region::Apulia,
                &[
                    ("ZAP", 19, RoadTypes::Asphalt),
                    ("APV", 17, RoadTypes::Cobblestone),
                    ("MTT", 14, RoadTypes::Asphalt),
                    ("CPI", 20, RoadTypes::Unpaved),
                    ("VIE", 24, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "MTT",
            City::new(
                "Mattina",
                Region::Apulia,
                &[
                    ("MFR", 14, RoadTypes::Asphalt),
                    ("VIE", 12, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "VIE",
            City::new(
                "Vieste",
                Region::Apulia,
                &[
                    ("MTT", 12, RoadTypes::Asphalt),
                    ("MFR", 24, RoadTypes::Cobblestone),
                    ("PCH", 11, RoadTypes::Asphalt),
                    ("CPI", 25, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "PCH",
            City::new(
                "Peschici",
                Region::Apulia,
                &[
                    ("VIE", 11, RoadTypes::Asphalt),
                    ("CPI", 18, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "CPI",
            City::new(
                "Carpino",
                Region::Apulia,
                &[
                    ("MFR", 20, RoadTypes::Unpaved),
                    ("VIE", 25, RoadTypes::Cobblestone),
                    ("PCH", 18, RoadTypes::Asphalt),
                    ("SSV", 21, RoadTypes::Unpaved),
                    ("CEU", 28, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "SSV",
            City::new(
                "San Severo",
                Region::Apulia,
                &[
                    ("LUC", 14, RoadTypes::Cobblestone),
                    ("CEU", 18, RoadTypes::Asphalt),
                    ("CPI", 21, RoadTypes::Cobblestone)
                ],
                false
            )
        ),
        (
            "CEU",
            City::new(
                "Chieuti",
                Region::Apulia,
                &[
                    ("SSV", 18, RoadTypes::Asphalt),
                    ("CPI", 28, RoadTypes::Asphalt),
                    ("LES", 13, RoadTypes::Asphalt)
                ],
                false
            )
        ),
        (
            "LES",
            City::new(
                "",
                Region::Apulia,
                &[
                    ("CEU", 13, RoadTypes::Asphalt),
                ],
                false
            )
        )
    ]);
}
