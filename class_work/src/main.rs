fn pattern_match_simple() {
    let by3 = num%3 == 0;
    let by5 = num%5 == 0;

    let result == match(by3, by5){
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        (false, false) => num.to_string(),
    };
    result
}

fn main(){
    for num in 1...20{
        println!("{}", pattern_match_simple(num));
    }
}