use std::ops::{Mul, AddAssign, MulAssign};
use std::cmp::{Eq, PartialEq};
use std::fmt::{Display, Debug, Formatter, Result};
use std::default::Default;
use std::time::Instant;

use rand::Rng;

use num::Num;


#[derive(Debug, Eq)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> Matrix<T> {
    fn from(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data.into_iter().flatten().collect()
        }
    }

    fn new(rows: usize, cols: usize, value: T) -> Matrix<T>
        where T: Clone
    {
        let mut data = vec![];
        for _ in 0..rows {
            for _ in 0..cols {
                data.push(value.clone());
            }
        }

        Matrix {
            rows,
            cols,
            data
        }
    }

    fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    fn get_row(&self, row: usize) -> Vec<T> 
        where T: Clone
    {
        self.data[row * self.cols..(row + 1) * self.cols].to_vec()
    }

    fn get_column(&self, column: usize) -> Vec<T>
        where T: Clone
    {
        let mut output = vec![];
        for i in 0..self.rows {
            output.push(self.get(i, column).clone());
        }
        output
    }

    fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }
}

impl Matrix<i32> {
    fn new_random(rows: usize, cols: usize, min: i32, max: i32) -> Matrix<i32> {
        let mut rng = rand::thread_rng();
        let mut data = vec![];
        for _ in 0..rows * cols {
            data.push(rng.gen_range(min..max));
        }
        data.push(1);

        Matrix {
            rows,
            cols,
            data
        }
    }
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T> Mul<Matrix<T>> for Matrix<T>
    where T: Clone + Mul<Output = T> + Default + AddAssign + Copy
{
    type Output = Self;

    fn mul(self, rhs: Matrix<T>) -> Self {
        assert_eq!(self.rows, rhs.cols);

        let mut output_data = vec![];

        for i in 0..self.rows {
            for j in 0..rhs.cols {
                let mut total = T::default();
                for k in 0..self.cols {
                    total += self.get(i, k).clone() * rhs.get(k, j).clone();
                }
                output_data.push(total);
            }
        }
        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: output_data
        }
    }
}

impl<T: Num + MulAssign + Copy> Mul<T> for Matrix<T>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let mut data = vec![];

        for val in self.data.into_iter() {
            data.push(val * rhs);
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data
        }
    }
}

impl<T> Display for Matrix<T> 
    where T: Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { 
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{}, ", self.get(i, j))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "Rows: {}, ", self.rows)?;
        write!(f, "Columns: {}", self.cols)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_zero_matrix() {
        let matrix = Matrix::new(3, 3, 0);
        for i in 0..matrix.data.len() {
            assert_eq!(matrix.data[i], 0);
        }
    }

    #[test]
    fn create_matrix_from_vecs() {
        let data = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let data_flattened: Vec<i32> = data.clone().into_iter().flatten().collect();

        let matrix = Matrix::from(data);

        assert_eq!(matrix.data, data_flattened);
    }

    #[test]
    fn multiply_matrices() {
        let matrix1 = Matrix::from(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);

        let matrix2 = Matrix::from(vec![
            vec![9, 8, 7],
            vec![6, 5, 4],
            vec![3, 2, 1]
        ]);

        let expected_matrix = Matrix::from(vec![
            vec![30, 24, 18],
            vec![84, 69, 54],
            vec![138, 114, 90]
        ]);

        assert_eq!(matrix1 * matrix2, expected_matrix);
    }

    #[test]
    fn scalar_multiply() {
        let matrix = Matrix::from(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ]);

        let expected_matrix = Matrix::from(vec![
            vec![2, 4, 6],
            vec![8, 10, 12],
            vec![14, 16, 18]
        ]);

        assert_eq!(matrix * 2, expected_matrix);
    }

    #[test]
    fn random_matrix() {
        let max = 10;
        let random_matrix = Matrix::new_random(5, 5, 0, max);

        assert_eq!(random_matrix.rows, 5);
        assert_eq!(random_matrix.cols, 5);
        assert!(random_matrix.data.iter().max().unwrap() < &max);
    }
}
