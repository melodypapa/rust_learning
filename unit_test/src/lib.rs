fn sum(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn sum_inputs_outputs() -> Vec<(i8, i8) , i8){
        Vec!([(1,1 , 2)])
    }
}
