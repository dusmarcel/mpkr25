//use derive_more::Display;
//use yew::prelude::*;

//use std::collections::HashMap;

#[derive(Clone, PartialEq)]//, Display, PartialEq)]
pub enum Verfahren {
    Hauptsache,
    Vorlaeufig,
    Beides
}

#[derive(Clone, PartialEq)]//, Display, PartialEq, Properties)]
pub struct Mpkr {
    pub verfahren: Verfahren,
}

impl Mpkr {
    pub fn new() -> Self {
        Self {
            verfahren: Verfahren::Hauptsache
        }
    }

    pub fn set_verfahren(&mut self, verfahren: Verfahren) {
        self.verfahren = verfahren;
    }

    pub fn get_props(&self) -> String {
        //let mut props = HashMap::new();
        format!("v={}", match self.verfahren {
            Verfahren::Hauptsache => "h",
            Verfahren::Vorlaeufig => "v",
            Verfahren::Beides => "b"
        })
        // props.insert(
        //     "v".to_string(),
        //     match self.verfahren {
        //         Verfahren::Hauptsache => "h".to_string(),
        //         Verfahren::Vorlaeufig => "v".to_string(),
        //         Verfahren::Beides => "b".to_string()
        //     }
        // );
        // props
    }
}