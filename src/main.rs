mod matrice;
mod equation_solver;

use matrice::*;

fn main() {
    println!("Equation system solver");
    println!("
Please insert your matrix using spaces to separate elements and new lines to separate rows.
Use a period for decimals. The last element in the row will be seen as the sum.");

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

fn input () -> AugMatrice
{
    let mut rows : Vec<Row> = Vec::new();
    let mut line = String::new();
    loop {
        match read_line(&mut line) {
            Ok(row) => {
                if rows.len() == 0 || row.elements.len() == rows[0].elements.len() { rows.push(row); }
                else { println!("You failed to enter a row of the same length, try again."); }
            },
            Err(error) => {
                match error {
                    MatriceError::LineEmpty => break,
                    _ => println!("Row input was incorrect, please try again.")}
                }
        }
        line.clear();
    }

    return AugMatrice::new(rows);
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
