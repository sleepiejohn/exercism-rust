pub fn build_proverb(list: Vec<&str>) -> String {
    let body = list.windows(2).map(to_phrase).collect::<String>();
    let ending = match list.len() {
        0 => "",
        s if s > 3 && s < 5 => "And all for the want of a pin.",
        _ => "And all for the want of a nail.",
        
    };
    format!("{}{}", body, ending)
}

fn to_phrase(words: &[&str]) -> String {
    format!("For want of a {} the {} was lost.\n", words[0], words[1])
}
