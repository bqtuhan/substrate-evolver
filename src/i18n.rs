// File: src/i18n.rs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    En,
    Tr,
}

pub struct I18n {
    pub lang: Language,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }

    pub fn set_language(&mut self, lang: Language) {
        self.lang = lang;
    }

    pub fn label_tick(&self) -> &'static str {
        match self.lang {
            Language::En => "Tick",
            Language::Tr => "Tur",
        }
    }

    pub fn label_agents(&self) -> &'static str {
        match self.lang {
            Language::En => "Agents",
            Language::Tr => "Ajan",
        }
    }

    pub fn label_phase(&self) -> &'static str {
        match self.lang {
            Language::En => "Phase",
            Language::Tr => "Faz",
        }
    }

    pub fn label_avg_speed(&self) -> &'static str {
        match self.lang {
            Language::En => "Avg Spd",
            Language::Tr => "Ort Hız",
        }
    }

    pub fn label_avg_vision(&self) -> &'static str {
        match self.lang {
            Language::En => "Avg Vis",
            Language::Tr => "Ort Gör",
        }
    }

    pub fn label_food(&self) -> &'static str {
        match self.lang {
            Language::En => "Food",
            Language::Tr => "Yem",
        }
    }

    pub fn ice_age_active_text(&self) -> &'static str {
        match self.lang {
            Language::En => "[!! ICE AGE ACTIVE !!]",
            Language::Tr => "[!! BUZ ÇAĞI AKTİF !!]",
        }
    }

    pub fn simulation_ended(&self) -> &'static str {
        match self.lang {
            Language::En => "All agents died. Simulation ended.",
            Language::Tr => "Tüm ajanlar öldü. Simülasyon sona erdi.",
        }
    }
}