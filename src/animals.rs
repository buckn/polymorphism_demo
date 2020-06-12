mod cat;
mod dog;

use cat::Cat;
use dog::Dog;

pub trait Animal {
    fn is_dog(&self) -> bool;

    fn get_name(&self) -> &String;
    fn get_weight(&self) -> f64;
    fn get_breed(&self) -> String;

    fn set_breed(&mut self, x: String);
    fn set_name(&mut self, x: String);
    fn set_weight(&mut self, x: f64);

    fn to_string(&self) -> String;
}

pub struct AnimalVec {
    animal_vec: Vec<Box<dyn Animal>>,
}

impl AnimalVec {
    pub fn new() -> Self {
        Self { animal_vec: vec![] }
    }
    pub fn add(&mut self, is_dog: bool) {
        if is_dog {
            self.animal_vec.push(Box::new(Dog::new()));
        } else {
            self.animal_vec.push(Box::new(Cat::new()));
        }
    }
    pub fn to_string(&self) -> String {
        let mut return_string = "".to_string();
        for item in &self.animal_vec {
            return_string += &(item.to_string() + "\n\n")
        }
        return_string += "\n\n\n";
        return_string
    }
    pub fn dog_cat_ratio(&self) -> i32 {
        let mut dogs = 0;
        let mut cats = 0;
        for item in &self.animal_vec {
            if item.is_dog() {
                dogs = dogs + 1;
            } else {
                cats = cats + 1;
            }
        }
        if cats == 0 {
            return -100;
        }
        dogs / cats
    }
}
