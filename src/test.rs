#[cfg(test)]
use super::*;

#[test]
fn define_conversion_type_works() {
    let mut input = String::from("F");
    assert_eq!("C", define_conversion_type(&input));


    input.push('f');
    assert_eq!("C", define_conversion_type(&input));


    input.push('C');
    assert_eq!("F", define_conversion_type(&input));

    input.push('c');
    assert_eq!("F", define_conversion_type(&input));
}


#[test]
fn get_f64_from_input_works() {
    assert_eq!(1.1, get_f64_from_input(&("1.1C").to_string()));
    assert_eq!(1.999,get_f64_from_input(&("1.999f").to_string()));
    assert_eq!(8.0,get_f64_from_input(&("8C").to_string()));
}

#[test]
fn convert_works(){
    assert_eq!(77.0,convert(25.0,&"F"));
}