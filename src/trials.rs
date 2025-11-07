#[derive(PartialEq, Clone)]
pub struct Trial {
    name: String,
    base_score: u32,
    max_vitality: u8,
    score_factor: f64,
    hardmodes: Vec<Hardmode>,
}

impl Trial {
    pub fn get_name(&self) -> String {
        if matches!(self.name.as_str(), "Aetherian Archive" | "Hel Ra Citadel" | "Sanctum Ophidia") {
            return self.name.clone()
        } else {
            return format!("{} (VETERAN)", self.name.clone())
        }
    }

    pub fn get_raw_name(&self) -> String {
        return self.name.clone();
    }

    pub fn calculate_score(&self, time: u32) -> u32 {
        let vitality_bonus = self.max_vitality;
        return self.calculate_score_with_vitality(time, vitality_bonus)
    }

    pub fn calculate_score_with_vitality(&self, time: u32, vitality: u8) -> u32 {
        let vitality_bonus = self.get_vitality_bonus_with_vitality(vitality);
        let score = self.base_score + self.hardmodes.iter().map(|h| h.additional_score).sum::<u32>();
        let calculated_score = (score + vitality_bonus) as f64
        * (1.0 + (self.get_score_factor() - time as f64) / 10_000_000.0);
        return calculated_score as u32
    }

    pub fn calculate_time_from_score(&self, final_score: u32, vitality: u8) -> u32 {
        let vitality_bonus = self.get_vitality_bonus_with_vitality(vitality);
        let base_score = self.base_score + self.hardmodes.iter().map(|h| h.additional_score).sum::<u32>();
        let total_score = (base_score + vitality_bonus) as f64;

        let factor = self.get_score_factor();
        let ratio = final_score as f64 / total_score;

        let time = factor - 10_000_000.0 * (ratio - 1.0);
        time.max(0.0) as u32
    }


    pub fn get_vitality_bonus(&self) -> u32 {
        return self.get_vitality_bonus_with_vitality(self.max_vitality)
    }

    pub fn get_vitality_bonus_with_vitality(&self, vitality: u8) -> u32 {
        return vitality as u32 * 1000
    }

    pub fn get_maximum_vitality(&self) -> u8 {
        return self.max_vitality
    }

    pub fn get_score_factor(&self) -> f64 {
        return self.score_factor * 1000.0
    }
}

#[derive(PartialEq, Clone)]
struct Hardmode {
    name: String,
    additional_score: u32,
}

pub fn create_trial_structs() -> Vec<Trial> {
    let trials = vec![
        Trial {
            name: "Aetherian Archive".to_string(),
            base_score: 84300,
            max_vitality: 24,
            score_factor: 900.0,
            hardmodes: vec![
                Hardmode {
                    name: "HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Hel Ra Citadel".to_string(),
            base_score: 93100,
            max_vitality: 24,
            score_factor: 900.0,
            hardmodes: vec![
                Hardmode {
                    name: "HM".to_string(),
                    additional_score: 40000,
                }
            ],
        },
        Trial {
            name: "Sanctum Ophidia".to_string(),
            base_score: 102700,
            max_vitality: 24,
            score_factor: 1500.0,
            hardmodes: vec![
                Hardmode {
                    name: "HM".to_string(),
                    additional_score: 40000,
                }
            ],
        },
        Trial {
            name: "Maw of Lorkhaj".to_string(),
            base_score: 68150,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "HM".to_string(),
                    additional_score: 40000,
                }
            ],
        },
        Trial {
            name: "Halls of Fabrication".to_string(),
            base_score: 120100,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "HM".to_string(),
                    additional_score: 40000,
                }
            ],
        },
        Trial {
            name: "Asylum Sanctorium".to_string(),
            base_score: 15000,
            max_vitality: 36,
            score_factor: 1200.0,
            hardmodes: vec![
                Hardmode {
                    name: "+1".to_string(),
                    additional_score: 15000,
                },
                Hardmode {
                    name: "+2".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Cloudrest".to_string(),
            base_score: 18000,
            max_vitality: 36,
            score_factor: 1200.0,
            hardmodes: vec![
                Hardmode {
                    name: "+1".to_string(),
                    additional_score: 14250
                },
                Hardmode {
                    name: "+2".to_string(),
                    additional_score: 14250,
                },
                Hardmode {
                    name: "+3".to_string(),
                    additional_score: 39250,
                },
                Hardmode {
                    name: "+3 & Trash".to_string(),
                    additional_score: 2250,
                },
            ],
        },
        Trial {
            name: "Sunspire".to_string(),
            base_score: 87250,
            max_vitality: 36,
            score_factor: 1800.0,
            hardmodes: vec![
                Hardmode {
                    name: "Yolnahkriin HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Lokkestiiz HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Nahviintaas HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Kyne's Aegis".to_string(),
            base_score: 85950,
            max_vitality: 36,
            score_factor: 1200.0,
            hardmodes: vec![
                Hardmode {
                    name: "Yandir HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Vrol HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Falgravn HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Rockgrove".to_string(),
            base_score: 112200,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "Oaxiltso HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Bahsei HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Xalvakka HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Dreadsail Reef".to_string(),
            base_score: 145850,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "Twins HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Reef HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Taleria HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Sanity's Edge".to_string(),
            base_score: 85200,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "Yaseyla HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Chimera HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Ansuul HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Lucent Citadel".to_string(),
            base_score: 72850,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "Twins HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Orphic HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Xoryn HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Ossein Cage".to_string(),
            base_score: 108550,
            max_vitality: 36,
            score_factor: 2700.0,
            hardmodes: vec![
                Hardmode {
                    name: "Shapers HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Twins HM".to_string(),
                    additional_score: 40000,
                },
                Hardmode {
                    name: "Kazpian HM".to_string(),
                    additional_score: 40000,
                },
            ],
        },
        Trial {
            name: "Dragonstar Arena".to_string(),
            base_score: 20000,
            max_vitality: 24,
            score_factor: 3600.0,
            hardmodes: vec![],
        },
        Trial {
            name: "Maelstrom Arena".to_string(),
            base_score: 426000,
            max_vitality: 15,
            score_factor: 5400.0,
            hardmodes: vec![],
        },
        Trial {
            name: "Blackrose Prison".to_string(),
            base_score: 75000,
            max_vitality: 24,
            score_factor: 2400.0,
            hardmodes: vec![],
        },
        Trial {
            name: "Vateshran Hollows".to_string(),
            base_score: 205550,
            max_vitality: 15,
            score_factor: 5400.0,
            hardmodes: vec![],
        },
    ];

    return trials
}