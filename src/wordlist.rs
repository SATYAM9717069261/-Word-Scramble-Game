use rand::seq::SliceRandom;

pub fn generate_word_list() -> &'static str {
    let words = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "pineapple",
        "quince",
        "raspberry",
        "strawberry",
        "tangerine",
        "ugli",
        "vanilla",
        "watermelon",
    ];
    let mut rng = rand::thread_rng();
    return words.choose(&mut rng).unwrap();
}
