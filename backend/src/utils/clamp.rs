pub fn clamp<T: PartialOrd>(min: T, max: T, value: T) -> T {
    if value < min { min }
    else if value > max { max }
    else { value }
}