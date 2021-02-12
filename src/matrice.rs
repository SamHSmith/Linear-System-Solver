use MatriceError::*;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct AugMatrice {
    rows : Vec<Row>,
    width : usize, //Does not inclued augmented matrix
    height : usize,
}

impl AugMatrice {
    pub fn new(rows : Vec<Row>) -> Result<AugMatrice, MatriceError> {
        let height = rows.len();
        
        let mut rows_iter = rows.iter();
        let width = match rows_iter.next() {
            Some(first_row) => first_row.elements.len(),
            None => return Ok(AugMatrice{rows : rows, width : 0, height : 0 })
        };

        for i in rows_iter {
            if i.elements.len() != width {return Err(RowsNotOfEqualLength);}
        }

        Ok(AugMatrice{rows : rows, width : width, height : height})
    }

    pub fn height(&self) -> usize { //Make sure this function is used everwhere it should!!!!
        self.height
    }

    pub fn width(&self) -> usize { //Make sure this function is used everwhere it should!!!!
        self.width
    }
    
    pub fn get_element(&self, row : usize, element : usize) -> Result<f64, MatriceError> {
        Ok(
        *self.rows.get(row).ok_or(IndexOutOfBounds)?
        .elements.get(element).ok_or(IndexOutOfBounds)?
        )
    }

    pub fn get_sum(&self, row : usize) -> Result<f64, MatriceError> {
        Ok(self.get_row(row)?.sum)
    }

    pub fn row_switch(&mut self, row1 : usize, row2 : usize) -> Result<(), MatriceError> {
        if row1 >= self.height() || row2 >= self.height() {return Err(IndexOutOfBounds)}

        self.rows.swap(row1, row2);

        Ok(())
    }

    pub fn multiply_row(&mut self, row : usize, factor : f64) -> Result<(), MatriceError> {
        let row = self.get_row_mut(row)?;

        row.multiply_row(factor);
        row.sum *= factor;

        Ok(())
    }

    pub fn add_multiplied_row(&mut self, row1 : usize, row2 : usize, factor : f64) -> Result<(), MatriceError> {
        let (row1, row2) = self.two_mut_row(row1, row2)?;

        row2.add_rows(row1, factor);

        Ok(())
    }

    fn get_row_mut(&mut self, row : usize) -> Result<&mut Row, MatriceError> {
        Ok(self.rows.get_mut(row).ok_or(MatriceError::IndexOutOfBounds)?)
    }

    fn get_row(&self, row : usize) -> Result<&Row, MatriceError> {
        Ok(self.rows.get(row).ok_or(MatriceError::IndexOutOfBounds)?)
    }

    fn two_mut_row(&mut self, row1 : usize, row2 : usize) -> Result<(&mut Row, &mut Row), MatriceError> {
        if row1 == row2 {return Err(CannotGetSameReference)}
        if row1 >= self.height() || row2 >= self.height {return Err(IndexOutOfBounds)} 

        Ok(two_mut_ref(&mut self.rows, row1, row2))
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Row {
    elements : Vec<f64>,
    sum : f64
}

impl Row {
    pub fn new(row : Vec<f64>, sum : f64) -> Row {
        Row{elements : row, sum : sum}
    }

    fn multiply_row(&mut self, factor : f64) {
        for i in self.elements.iter_mut() {
            *i *= factor;
        }
    }

    fn add_rows(&mut self, factor_row : &Row, factor : f64) {
        self.sum += factor_row.sum * factor;

        let row = self.elements.iter_mut();
        let factor_row = factor_row.elements.iter();

        let rows = row.zip(factor_row);

        for (element, factor_element) in rows {
            *element += factor_element * factor;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MatriceError {
    IndexOutOfBounds,
    CannotGetSameReference,
    RowsNotOfEqualLength,
    SystemIsUnsolvable,
    CouldNotParseToNumber,
    LineEmpty,
}


mod test {
    use super::*;
    #[test]
    fn row_switch_test() {
        let mut  matrice = AugMatrice{rows : vec![
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![9.0,10.0,11.0], sum : 12.0},
        ], width : 3, height : 3};

        matrice.row_switch(0, 1).unwrap();

        assert_eq!(matrice, AugMatrice{rows : vec![
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![9.0,10.0,11.0], sum : 12.0},
        ], width : 3, height : 3})
    }

    #[test]
    fn multiply_row_test() {
        let mut  matrice = AugMatrice{rows : vec![
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![9.0,10.0,11.0], sum : 12.0},
        ], width : 3, height : 3};

        matrice.multiply_row(2, 2f64).unwrap();

        assert_eq!(matrice, AugMatrice{rows : vec![
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![18.0,20.0,22.0], sum : 24.0},
        ], width : 3, height : 3})
    }

    #[test]
    fn add_multiplied_row() {
        let mut  matrice = AugMatrice{rows : vec![
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![9.0,10.0,11.0], sum : 12.0},
        ], width : 3, height : 3};

        matrice.add_multiplied_row(1, 2, 2f64).unwrap();

        assert_eq!(matrice, AugMatrice{rows : vec![
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![19.0,22.0,25.0], sum : 28.0},
        ], width : 3, height : 3})
    }

    #[test]
    fn new_matrice_test1() {
        let matrice = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![5.0, 6.0, 7.0], 8.0),
            Row::new(vec![9.0, 10.0, 11.0], 12.0),
        ]).unwrap();
        
        assert_eq!(matrice, AugMatrice{rows : vec![
            Row{elements : vec![1.0,2.0,3.0], sum : 4.0},
            Row{elements : vec![5.0,6.0,7.0], sum : 8.0},
            Row{elements : vec![9.0,10.0,11.0], sum : 12.0},
        ], width : 3, height : 3})
    }

    #[test]
    fn new_matrice_test2() {
        let matrice = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0], 4.0),
            Row::new(vec![5.0,6.0,7.0], 8.0),
            Row::new(vec![9.0,10.0,11.0], 12.0),
        ]);
        
        assert_eq!(matrice, Err(MatriceError::RowsNotOfEqualLength))
    }

}

// <Not mine>
fn two_mut_ref(slice: &mut [Row], idx1: usize, idx2: usize) -> (&mut Row, &mut Row) {
    assert!(idx1 != idx2);

    // Determine which index is higher    
    if idx1 < idx2 {
        // Use the higher index for the split,
        // so that the lower index will be somewhere in the lower slice
        // and the higher index will be first in the higher slice
        let (lower, higher) = slice.split_at_mut(idx2);
        
        (&mut lower[idx1], &mut higher[0])
    } else {
        let (lower, higher) = slice.split_at_mut(idx1);
        
        (&mut higher[0], &mut lower[idx2])
    }
}
// </Not mine>