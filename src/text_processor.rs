pub fn replace_characters(replacements:(char,char),original_text:&String) -> String
{
    let mut modified_text:String = String::new();
    for mut letter in original_text.chars() {
        if letter == replacements.0 {
            letter = replacements.1
        }
        modified_text.push(letter)
    }
    modified_text
}