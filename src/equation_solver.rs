use super::matrice::*;

impl AugMatrice {
    pub fn solve_matrice(mut self) -> Result<Vec<f64>, MatriceError> {
        if self.width() > self.height() {
            return Err(MatriceError::SystemIsUnsolvable);
        }

        for iteration in 0usize..self.width() {
            self.reorder_row(iteration)?;
        
            self.make_leading_one(iteration);
            
            self.clean_column(iteration);
            
        };
        
        match self.is_solved() {
            true => Ok(self.get_solution()),
            false => return Err(MatriceError::SystemIsUnsolvable),
        }        
    }

    fn reorder_row(&mut self, iteration : usize) -> Result<(), MatriceError> { //Change error messages
        for row in iteration..self.height() {
            if self.get_element(row, iteration).expect("Element out of range, should not be possible ErrID : 0") != 0f64 {
                self.row_switch(iteration, row).expect("Row out of range, should not be possible ErrID : 1");
                
                return Ok(());
            }
        }

        Err(MatriceError::SystemIsUnsolvable)
    }

    fn make_leading_one(&mut self, iteration : usize) {
        let leading_non_zero = self.get_element(iteration, iteration).expect("Element out of range, should not be possible ErrID : 2");
        self.multiply_row(iteration, 1f64 / leading_non_zero).expect("Row out of range, should not be possible ErrID : 3");
    }

    fn clean_column(&mut self, iteration : usize) {
        for row in (0..iteration).chain(iteration + 1.. self.height()) { //makes an iterator with all but the column value in it
            let factor = -self.get_element(row, iteration).expect("Element out of range, should not be possible ErrID : 4");
            self.add_multiplied_row(iteration, row, factor).expect("Element out of range, should not be possible ErrID : 5");
        }
    }

    fn is_solved(&mut self) -> bool {
        for row in self.width()..self.height() {
            if self.get_sum(row).expect("Element out of range, should not be possible ErrID : 7") != 0f64 {
                return false
            }
        }

        true
    }

    fn get_solution(self) -> Vec<f64> {
        let mut solution = Vec::with_capacity(self.width());
        for row in 0..self.width() {
            solution.push(self.get_sum(row).expect("Row out of range, should not be possible ErrID : 6"));
        }

        solution
    }
}

mod test {
    use super::*;

    #[test]
    fn reorder_row_test1() {
        let mut matrice = AugMatrice::new(vec![
            Row::new(vec![0.0, 2.0, 3.0], 4.0),
            Row::new(vec![5.0, 6.0, 7.0], 8.0),
            Row::new(vec![9.0, 10.0, 11.0], 12.0),
        ]).unwrap();

        matrice.reorder_row(0).unwrap();

        let matrice_new = AugMatrice::new(vec![
            Row::new(vec![5.0, 6.0, 7.0], 8.0),
            Row::new(vec![0.0, 2.0, 3.0], 4.0),
            Row::new(vec![9.0, 10.0, 11.0], 12.0),
        ]).unwrap();

        assert_eq!(matrice, matrice_new)
    }

    #[test]
    fn reorder_row_test2() {
        let mut matrice = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![0.0, 0.0, 7.0], 8.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
        ]).unwrap();

        matrice.reorder_row(1).unwrap();

        let matrice_new = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
            Row::new(vec![0.0, 0.0, 7.0], 8.0),
        ]).unwrap();

        assert_eq!(matrice, matrice_new)
    }

    #[test]
    fn reorder_row_test3() {
        let mut matrice = AugMatrice::new(vec![
            Row::new(vec![0.0, 2.0, 3.0], 4.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
            Row::new(vec![0.0, 0.0, 7.0], 8.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
        ]).unwrap();

        let error = matrice.reorder_row(0);
        
        assert_eq!(error, Err(MatriceError::SystemIsUnsolvable))
    }

    #[test]
    fn make_leading_one_test() {
        let mut matrice = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
            Row::new(vec![0.0, 0.0, 7.0], 8.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
        ]).unwrap();

        matrice.make_leading_one(1);

        let matrice_new =  AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![0.0, 1.0, 1.1], 1.2000000000000002),
            Row::new(vec![0.0, 0.0, 7.0], 8.0),
            Row::new(vec![0.0, 10.0, 11.0], 12.0),
        ]).unwrap();
        
        assert_eq!(matrice, matrice_new)
    }

    #[test]
    fn clean_column_test() {
        let mut matrice = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![3.0, 10.0, 11.0], 12.0),
            Row::new(vec![2.0, 0.0, 7.0], 8.0),
            Row::new(vec![4.0, 10.0, 11.0], 12.0),
        ]).unwrap();

        matrice.clean_column(0);

        let matrice_new =  AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 3.0], 4.0),
            Row::new(vec![0.0, 4.0, 2.0], 0.0),
            Row::new(vec![0.0, -4.0, 1.0], 0.0),
            Row::new(vec![0.0, 2.0, -1.0], -4.0),
        ]).unwrap();
        
        assert_eq!(matrice, matrice_new)
    }

    #[test]
    fn solve_matrice_test1() {
        let matrice = AugMatrice::new(vec![
            Row::new(vec![1.0, 2.0, 0.0], 5.0),
            Row::new(vec![0.0, 0.0, 1.0], 3.0),
            Row::new(vec![2.0, 1.0, 0.0], 4.0),
        ]).unwrap();

        let solution = matrice.solve_matrice().unwrap();
        
        assert!(solution.iter().map(|x| *x as u32).eq(vec![1,2,3].iter().map(|x| *x as u32)))
    }
}