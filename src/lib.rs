
pub fn raindrops(n: i64) -> String {

    //Find the factors of the raindrop numbers parsed
    let factor = find_factors(n as i128);

    // Establish the return varible "number"
    let mut number = String::new();

    //Create the raindrops String
    for i in factor.iter() {
        number = number.clone() + &String::from(match_string(&i));
    };

    //Return String result
    String::from(number)
}

//get the raindrop String
fn match_string(a: &i128) -> String {
    let number = match a {
        3 => String::from("Pling"),
        5 => String::from("Plang"),
        7 => String::from("Plong"),
        _ => a.to_string(),
    };
    number
}

// find a numbers factors
fn find_factors(a: i128) -> Vec<i128> {
    // create a new vector to hold returned factors
    let mut found_factors = Vec::new();

    if a <= 8 { // Work around for none loop factors
        found_factors.push(a);
        return found_factors
    }

    for index in 3..a  {
        let b = a % index;
        if b == 0 && index <= 7{
            if index == 3 || index == 5 || index == 7 {
                found_factors.push(index);
            } else {
                found_factors.push(a);
            }
        }
    }

    if found_factors.len() == 0 {
        found_factors.push(a);
    }

    found_factors
}
