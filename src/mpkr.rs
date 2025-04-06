use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum Verfahren {
    Hauptsache,
    Vorlaeufig,
    Beides
}

#[derive(Clone, Serialize)]
pub struct Mpkr {
    #[serde(default="Verfahren::Hauptsache")]
    pub v: Verfahren,
}

impl Mpkr {
    pub fn new() -> Self {
        Self {
            v: Verfahren::Hauptsache
        }
    }

    pub fn get_verf_string(&self) -> String {
        match self.v {
            Verfahren::Hauptsache => "Hauptsacheverfahren".to_string(),
            Verfahren::Vorlaeufig => "vorläufiger Rechtsschutz".to_string(),
            Verfahren::Beides => "Hauptsacheverfahren".to_string()
        }
    }
}