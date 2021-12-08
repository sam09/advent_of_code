use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub fn read_input_int(filename: &str)-> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();


    Ok(lines.map( |a| {
        a.unwrap().parse::<i32>().unwrap()
    } ).collect())
}

pub fn read_input_int64(filename: &str)-> io::Result<Vec<i64>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();


    Ok(lines.map( |a| {
        a.unwrap().parse::<i64>().unwrap()
    } ).collect())
}

pub fn read_input_string(filename: &str)-> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();


    Ok(lines.map( |a| {
        a.unwrap()
    } ).collect())
}

pub fn read_input_char_vec(filename: &str)-> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map( |a| {
        a.unwrap().chars().collect()
    } ).collect())
}

pub fn convert_string_to_int_vec(a: &String, delimiter: char) -> Vec<i32> {
    a.trim().split(delimiter).filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

pub fn convert_string_to_usize_vec(a: &String, delimiter: char) -> Vec<usize> {
    a.trim().split(delimiter).filter(|x| !x.is_empty()).map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()
}