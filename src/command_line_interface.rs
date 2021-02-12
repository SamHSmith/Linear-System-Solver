use super::matrice::*;
use super::equation_solver;

use crate::matrice::AugMatrice;

pub fn run() {
    println!("Equation system solver");
    println!("Please insert your matrix using spaces to separate elements and new lines to separate rows. Use a period for decimals. The last element in the row will be seen as the sum");

    let input = input();

    let solution = input.solve_matrice();
    match solution {
        Err(_) => println!("The system is unsolvable."),
        Ok(solution) => {
            println!("The following solution was found");
            for (i, &x) in solution.iter().enumerate() {
                println!("X{} = {}", i, x)
            }
        }
    }
}

fn input () -> AugMatrice {
    loop {
        let mut rows : Vec<Row> = Vec::new();
        let mut line = String::new();
        loop {
            match read_line(&mut line) {
                Ok(row) => rows.push(row),
                Err(error) => {
                    match error {
                        MatriceError::LineEmpty => break,
                        _ => println!("Row input was incorrect, please try again.")}
                    }
            }
            line.clear();
        }

        match AugMatrice::new(rows) {
            Ok(matrice) => return matrice,
            Err(_) => {
                println!("Rows were of unequal lenght. Please try again.")
            }
        }
        
    }
}

fn read_line(string : &mut String) -> Result<Row, MatriceError> { //Make it trim in place    
    std::io::stdin().read_line(string);

    let string = string.trim();

    if string == "" {return Err(MatriceError::LineEmpty)} 

    let string : Result<Vec<f64>, _ > = string.split(" ").map(|x| x.parse::<f64>()).collect();

    let string = match string {
        Ok(line) => line,
        Err(_) => return Err(MatriceError::CouldNotParseToNumber),
    };

    let string_len = string.len(); 
    
    let elements = string[0..string_len -1].to_vec();
    let sum = string[string_len -1];

    Ok(Row::new(elements, sum))
}