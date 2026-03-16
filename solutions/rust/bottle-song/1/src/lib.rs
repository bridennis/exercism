pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let text_words = [
        "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];

    let mut str = String::new();
    for i in ((start_bottles - take_down + 1)..=start_bottles).rev() {
        if i != start_bottles {
            str += "\n";
        }

        for _ in 1..=2 {
            str += &format!(
                "{} green bottle{} hanging on the wall,\n",
                text_words[i as usize][0..1].to_uppercase() + &text_words[i as usize][1..],
                if i == 1 { "" } else { "s" }
            );
        }

        str += &format!(
            "And if one green bottle should accidentally fall,\nThere'll be {} green bottle{} hanging on the wall.\n",
            text_words[(i - 1) as usize],
            if i - 1 == 1 { "" } else { "s" }
        );
    }

    str
}
