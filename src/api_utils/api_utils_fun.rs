use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

// 1. Definir la expresión regular para caracteres especiales permitidos.
// Acepta alfanuméricos, guion bajo y arroba.
lazy_static! {
    static ref RE_SPECIAL_CHARS: Regex = Regex::new(r"^[a-zA-Z0-9_@]+$").unwrap();
}

// 2. Función de validación personalizada para usar en el atributo.
pub fn validate_special_chars(value: &str) -> Result<(), ValidationError> {
    if !RE_SPECIAL_CHARS.is_match(value) {
        return Err(ValidationError::new(
            "Invalid characters: only letters, numbers, underscores, and @ are allowed",
        ));
    }
    Ok(())
}
