mod animals;

#[cfg(test)]
mod tests {
    use crate::animals::AnimalVec;
    #[test]
    fn to_string_test() {
        let mut x = AnimalVec::new();
        for i in 0..10 {
            x.add(match i % 2 {
                0 => true,
                _ => false,
            })
        }
        println!("{}", x.to_string());
    }
    #[test]
    fn dog_cat_ratio_test() {
        let mut x = AnimalVec::new();
        for i in 0..10 {
            x.add(match i % 2 {
                0 => true,
                _ => false,
            })
        }
        let y = x.dog_cat_ratio();
        println!("Dog to Cat Ratio: {}", y);
    }
}
