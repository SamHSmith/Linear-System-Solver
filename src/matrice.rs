use MatriceError::*;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct AugMatrice {
    rows : Vec<Row>,
    width : usize, //Does not include final column of the augmented matrix
    height : usize,
}

impl AugMatrice {
    pub fn new(rows : Vec<Row>) -> AugMatrice {
        let height = rows.len();
        if height == 0
        {
            return AugMatrice{rows : rows, width : 0, height : 0 };
        }
        
        let width = rows[0].elements.len();

        for i in rows.iter() {
            debug_assert!(i.elements.len() == width); // Compiles away in release mode
        }
        /*
            Instead of performing runtime checks for input argument validity we use a debug assert.
            This way we give the callee of the function the responsibility of passing non-junk data
            and we streamline the code by removing the error case. If during development the function
            is missused we simply crash through the debug assert.
        */

        AugMatrice{rows : rows, width : width, height : height}
    }


    pub fn height(&self) -> usize { //Make sure this function is used everwhere it should!!!!
        self.height
    }

    pub fn width(&self) -> usize { //Make sure this function is used everwhere it should!!!!
        self.width
    }

    pub fn get_element(&self, row : usize, element : usize) -> f64
    {
        debug_assert!(row < self.height(), element < self.width());

        self.rows[row].elements[element]
    }

    pub fn get_sum(&self, row : usize) -> f64 {
        debug_assert!(row < self.height);

        self.rows[row].sum
    }

    pub fn row_switch(&mut self, row1 : usize, row2 : usize) {
        debug_assert!(row1 < self.height, row2 < self.height);

        self.rows.swap(row1, row2);
    }

    pub fn multiply_row(&mut self, row : usize, factor : f64) {
        /*
            let row = self.get_row_mut(row)?;
            It's not our job to runtime check the callee. Instead we debug assert for our sanity.
            The following assert makes it a lot more clear to the reader what they
            did wrong to cause a crash.
        */
        debug_assert!(row < self.rows.len());

        self.rows[row].multiply_row(factor);
        // self.rows[row].sum *= factor; To justify the narly function call above I moved
        // this into Row::multiply_row
    }

    pub fn add_multiplied_row(&mut self, row1 : usize, row2 : usize, factor : f64)
    {
        debug_assert!(row1 < self.height(), row2 < self.height());
        /*
        let row1 = self.rows.get(row1).unwrap();   // We know these succeed if the assert passes
        let row2 = self.rows.get_mut(row2).unwrap();
        But rust won't let us use them because borrow checker. :(
        Instead we can index into the array on each time, not doing any borrows,
        thereby skipping the checking.
        */

        for i in 0..self.width
        {
            self.rows[row2].elements[i] += self.rows[row1].elements[i] * factor;
        }
        self.rows[row2].sum += self.rows[row1].sum * factor;

        //row2.add_rows(row1, factor); this doesn't work without that narly borrow code you borrowed.
    }

}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Row {
    pub elements : Vec<f64>,
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
        self.sum *= factor;
    }
/*
        You feel in the object oriented trap which required you to do the narly borrow.
        By moving this function up to the callee level we avoid the borrow situation.
    fn add_rows(&mut self, factor_row : &Row, factor : f64) {
        self.sum += factor_row.sum * factor;

        let row = self.elements.iter_mut();
        let factor_row = factor_row.elements.iter();

        let rows = row.zip(factor_row);

        for (element, factor_element) in rows {
            *element += factor_element * factor;
        }
    }
*/
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

