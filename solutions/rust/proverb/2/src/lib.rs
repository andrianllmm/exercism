pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut proverb = String::new();
    let n = list.len() - 1;

    for i in 0..n {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i],
            list[i+1]
        ));
    }

    proverb.push_str(&format!(
        "And all for the want of a {}.",
        list[0]
    ));

    return proverb;
}
