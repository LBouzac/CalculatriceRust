pub fn addition(a: i32, b: i32) -> i32 {
    a + b
}

pub fn carre(a: i32) -> i32 {
    a * a
}

pub fn soustraction(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

pub fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Erreur : Division par zéro !".to_string())
    } else {
        Ok(a / b)
    }
}
