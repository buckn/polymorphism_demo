use crate::animals::Animal;
use rand::Rng;

enum DogBreed {
    RETRIEVER = 0,
    BEAGLE = 1,
    BULLDOG = 2,
}

pub struct Dog {
    breed: DogBreed,
    weight: f64,
    name: String,
}

impl Dog {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            breed: match rng.gen_range(0, 3) {
                0 => DogBreed::RETRIEVER,
                1 => DogBreed::BEAGLE,
                _ => DogBreed::BULLDOG,
            },
            weight: rng.gen::<f64>() * 343.0,
            name: match rng.gen_range(0, 3) {
                0 => "Max".to_string(),
                1 => "Bailey".to_string(),
                _ => "Buddy".to_string(),
            },
        }
    }
}

impl Animal for Dog {
    fn is_dog(&self) -> bool {
        true
    }
    fn get_name(&self) -> &std::string::String {
        &self.name
    }
    fn get_weight(&self) -> f64 {
        self.weight
    }
    fn get_breed(&self) -> String {
        match self.breed {
            crate::animals::dog::DogBreed::RETRIEVER => "Retriever".to_string(),
            crate::animals::dog::DogBreed::BEAGLE => "Beagle".to_string(),
            crate::animals::dog::DogBreed::BULLDOG => "Bulldog".to_string(),
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
            "retriever" => crate::animals::dog::DogBreed::RETRIEVER,
            "beagle" => crate::animals::dog::DogBreed::BEAGLE,
            _ => crate::animals::dog::DogBreed::BULLDOG,
        }
    }
    fn to_string(&self) -> std::string::String {
        self.name.clone()
            + "\n    Canine"
            + "\n    weight: "
            + &self.weight.to_string()
            + "\n    breed: "
            + &self.get_breed()
    }
}
