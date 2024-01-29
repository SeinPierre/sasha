
fn main() {
    let result = sasha::open_binary("../nth_prime/target/release/nth_prime").unwrap();
    println!("{}",result.len());
}

#[cfg(test)]
mod tests {

    #[test]
    fn ut_read_simple_program(){
        let result = sasha::open_binary("../nth_prime/target/release/nth_prime");        
        assert_eq!(result.unwrap().len(),4407792);
    }
}