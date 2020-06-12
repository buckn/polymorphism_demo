use crate::animals::Animal;
use rand::Rng;

enum CatBreed {
    MAINECOON = 0,
    PERSIAN = 1,
    SPHYNX = 2,
}

pub struct Cat {
    breed: CatBreed,
    weight: f64,
    name: String,
}

impl Cat {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            breed: match rng.gen_range(0, 3) {
                0 => CatBreed::MAINECOON,
                1 => CatBreed::PERSIAN,
                _ => CatBreed::SPHYNX,
            },
            weight: rng.gen::<f64>() * 40.0,
            name: match rng.gen_range(0, 3) {
                0 => "Lucky".to_string(),
                1 => "Fluffy".to_string(),
                _ => "Leo".to_string(),
            },
        }
    }
}

impl Animal for Cat {
    fn is_dog(&self) -> bool {
        false
    }
    fn get_name(&self) -> &std::string::String {
        &self.name
    }
    fn get_weight(&self) -> f64 {
        self.weight
    }
    fn get_breed(&self) -> String {
        match &self.breed {
            crate::animals::cat::CatBreed::MAINECOON => "Maine Coon".to_string(),
            crate::animals::cat::CatBreed::PERSIAN => "Persian".to_string(),
            crate::animals::cat::CatBreed::SPHYNX => "Sphynx".to_string(),
        }
    }
    fn set_name(&mut self, x: std::string::String) {
        self.name = x;
    }
    fn set_weight(&mut self, x: f64) {
        self.weight = x;
    }
    fn set_breed(&mut self, x: String) {
        self.breed = match x.to_lowercase().as_str() {
            "persian" => crate::animals::cat::CatBreed::MAINECOON,
            "sphynx" => crate::animals::cat::CatBreed::PERSIAN,
            _ => crate::animals::cat::CatBreed::SPHYNX,
        }
    }
    fn to_string(&self) -> std::string::String {
        self.name.clone()
            + "\n    Feline"
            + "\n    weight: "
            + &self.weight.to_string()
            + "\n    breed: "
            + &self.get_breed()
    }
}
