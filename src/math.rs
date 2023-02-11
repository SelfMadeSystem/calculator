#[allow(non_snake_case)] // Math functions are usually written in camel case
pub fn nCr(n: f64, r: f64) -> f64 {
    return factorial(n) / (factorial(r) * factorial(n - r));
}

#[allow(non_snake_case)]
pub fn nPr(n: f64, r: f64) -> f64 {
    return factorial(n) / factorial(n - r);
}

pub fn factorial(n: f64) -> f64 {
    if n == 0.0 {
        return 1.0;
    }
    return n * factorial(n - 1.0);
}
