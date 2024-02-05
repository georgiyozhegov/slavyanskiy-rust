

// Of course, it isn't slavic alphabet, but at least russian
pub const ALPHABET: [char; 33] = [
    'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж',
    'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц',
    'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э', 'ю', 
    'я'
]; 


pub fn is_alpha(c: &char) -> bool {
    ALPHABET.contains(&c.to_lowercase().next().unwrap()) || *c == '_'
}
