pub fn clamp<T: Ord>(min: T, max: T, value: T) -> T {
    std::cmp::min(std::cmp::max(min, value), max)
} 