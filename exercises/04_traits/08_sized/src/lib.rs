pub fn example() {
    // Trying to get the size of a str (or any other DST)
    // via `std::mem::size_of` will result in a compile-time error.
    //
    // TODO: Comment out the following line and move on to the next exercise.
    let sz = std::mem::size_of::<&str>();
    println!("Size of sz {sz:?}")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kokoko() {
        example();
    }
}