#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 4, style: String::from("Sneakers") },
            Shoe { size: 4, style: String::from("Sneakers") },
            Shoe { size: 6, style: String::from("Sneakers") },
        ];

        let filtered_shoes = vec![
            Shoe { size: 4, style: String::from("Sneakers") },
            Shoe { size: 4, style: String::from("Sneakers") },
        ];

        assert_eq!(filtered_shoes, shoes_in_size(shoes, 4));
    }
}