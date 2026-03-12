use std::fmt;

/// geographic district with its  area code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Area {
    pub name: &'static str,
    pub area_code: &'static str,
}


impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.area_code)
    }
}


/// district/area code pair.
pub const AREAS: &[Area] = &[
    // ── Koshi Province ──
    Area { name: "Bhojpur",        area_code: "029" },
    Area { name: "Dhankuta",       area_code: "026" },
    Area { name: "Ilam",           area_code: "027" },
    Area { name: "Jhapa",          area_code: "023" },
    Area { name: "Khotang",        area_code: "036" },
    Area { name: "Morang",         area_code: "021" },
    Area { name: "Okhaldhunga",    area_code: "037" },
    Area { name: "Panchthar",      area_code: "024" },
    Area { name: "Sankhuwasabha",  area_code: "029" },
    Area { name: "Solukhumbu",     area_code: "038" },
    Area { name: "Sunsari",        area_code: "025" },
    Area { name: "Taplejung",      area_code: "024" },
    Area { name: "Terhathum",      area_code: "026" },
    Area { name: "Udayapur",       area_code: "035" },
    // ── Madhesh Province
    Area { name: "Bara",           area_code: "053" },
    Area { name: "Dhanusa",        area_code: "041" },
    Area { name: "Mahottari",      area_code: "044" },
    Area { name: "Parsa",          area_code: "051" },
    Area { name: "Rautahat",       area_code: "055" },
    Area { name: "Saptari",        area_code: "031" },
    Area { name: "Sarlahi",        area_code: "046" },
    Area { name: "Siraha",         area_code: "033" },
    // ── Bagmati Province
    Area { name: "Bhaktapur",      area_code: "01"  },
    Area { name: "Chitwan",        area_code: "056" },
    Area { name: "Dhading",        area_code: "010" },
    Area { name: "Dolakha",        area_code: "049" },
    Area { name: "Kathmandu",      area_code: "01"  },
    Area { name: "Kavrepalanchok", area_code: "011" },
    Area { name: "Lalitpur",       area_code: "01"  },
    Area { name: "Makwanpur",      area_code: "057" },
    Area { name: "Nuwakot",        area_code: "010" },
    Area { name: "Ramechhap",      area_code: "048" },
    Area { name: "Rasuwa",         area_code: "010" },
    Area { name: "Sindhupalchok",  area_code: "011" },
    Area { name: "Sindhuli",       area_code: "047" },
    // ── Gandaki Province 
    Area { name: "Baglung",        area_code: "068" },
    Area { name: "Gorkha",         area_code: "064" },
    Area { name: "Kaski",          area_code: "061" },
    Area { name: "Lamjung",        area_code: "066" },
    Area { name: "Manang",         area_code: "066" },
    Area { name: "Mustang",        area_code: "069" },
    Area { name: "Myagdi",         area_code: "069" },
    Area { name: "Nawalpur",       area_code: "078" },
    Area { name: "Parbat",         area_code: "067" },
    Area { name: "Syangja",        area_code: "063" },
    Area { name: "Tanahu",         area_code: "065" },
    // ── Lumbini Province 
    Area { name: "Arghakhanchi",   area_code: "077" },
    Area { name: "Banke",          area_code: "081" },
    Area { name: "Bardiya",        area_code: "084" },
    Area { name: "Dang",           area_code: "082" },
    Area { name: "Gulmi",          area_code: "079" },
    Area { name: "Kapilvastu",     area_code: "076" },
    Area { name: "Parasi",         area_code: "078" },
    Area { name: "Palpa",          area_code: "075" },
    Area { name: "Pyuthan",        area_code: "086" },
    Area { name: "Rolpa",          area_code: "086" },
    Area { name: "Rukum East",     area_code: "088" },
    Area { name: "Rupandehi",      area_code: "071" },
    // ── Karnali Province 
    Area { name: "Dailekh",        area_code: "089" },
    Area { name: "Dolpa",          area_code: "087" },
    Area { name: "Humla",          area_code: "019" },
    Area { name: "Jajarkot",       area_code: "089" },
    Area { name: "Jumla",          area_code: "087" },
    Area { name: "Kalikot",        area_code: "087" },
    Area { name: "Mugu",           area_code: "019" },
    Area { name: "Rukum West",     area_code: "088" },
    Area { name: "Salyan",         area_code: "088" },
    Area { name: "Surkhet",        area_code: "083" },
    // ── Sudurpashchim Province
    Area { name: "Achham",         area_code: "097" },
    Area { name: "Baitadi",        area_code: "095" },
    Area { name: "Bajhang",        area_code: "092" },
    Area { name: "Bajura",         area_code: "097" },
    Area { name: "Dadeldhura",     area_code: "096" },
    Area { name: "Darchula",       area_code: "093" },
    Area { name: "Doti",           area_code: "094" },
    Area { name: "Kailali",        area_code: "091" },
    Area { name: "Kanchanpur",     area_code: "099" },
];


/// Returns every [`Area`] whose `area_code` matches `code`.
pub fn areas_by_code(code: &str) -> Vec<&'static Area> {
    AREAS.iter().filter(|a| a.area_code == code).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_area() {
        let a = Area { name: "Kathmandu", area_code: "01" };
        assert_eq!(a.to_string(), "Kathmandu (01)");
    }

    #[test]
    fn areas_by_code_multiple_hits() {
        let names: Vec<&str> = areas_by_code("01").iter().map(|a| a.name).collect();
        assert!(names.contains(&"Kathmandu"));
        assert!(names.contains(&"Lalitpur"));
        assert!(names.contains(&"Bhaktapur"));
    }

    #[test]
    fn areas_by_code_no_hits() {
        assert!(areas_by_code("000").is_empty());
    }
}