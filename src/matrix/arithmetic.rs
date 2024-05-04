mod add;
mod mul;
mod sub;

use super::index::translate_index_between_orders_unchecked;
use super::iter::VectorIter;
use super::order::Order;
use super::shape::AxisShape;
use super::Matrix;
use crate::error::{Error, Result};

pub fn ensure_elementwise_operation_conformable<L, R>(
    lhs: &Matrix<L>,
    rhs: &Matrix<R>,
) -> Result<()> {
    if lhs.shape() != rhs.shape() {
        Err(Error::MatricesInconformable)
    } else {
        Ok(())
    }
}

pub fn ensure_multiplication_conformable<L, R>(lhs: &Matrix<L>, rhs: &Matrix<R>) -> Result<()> {
    if lhs.ncols() != rhs.nrows() {
        Err(Error::MatricesInconformable)
    } else {
        Ok(())
    }
}

pub fn vector_dot_product<L, R, T>(lhs: VectorIter<&L>, rhs: VectorIter<&R>) -> Option<T>
where
    L: std::ops::Mul<R, Output = T> + Clone,
    R: Clone,
    T: std::ops::Add<Output = T> + Default,
{
    lhs.zip(rhs)
        .map(|(x, y)| x.clone() * y.clone())
        .reduce(|acc, v| acc + v)
}

impl<L> Matrix<L> {
    pub fn elementwise_operation<R, T, F>(&self, rhs: &Matrix<R>, mut op: F) -> Result<Matrix<T>>
    where
        F: FnMut((&L, &R)) -> T,
    {
        ensure_elementwise_operation_conformable(self, rhs)?;

        let data = if self.order == rhs.order {
            self.data.iter().zip(rhs.data.iter()).map(op).collect()
        } else {
            self.data
                .iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index) };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_consume_rhs<R, T, F>(
        &self,
        rhs: Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<T>>
    where
        R: Clone,
        F: FnMut((&L, R)) -> T,
    {
        ensure_elementwise_operation_conformable(self, &rhs)?;

        let data = if self.order == rhs.order {
            self.data.iter().zip(rhs.data).map(op).collect()
        } else {
            self.data
                .iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index).clone() };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_consume_self<R, T, F>(
        self,
        rhs: &Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<T>>
    where
        F: FnMut((L, &R)) -> T,
    {
        ensure_elementwise_operation_conformable(&self, rhs)?;

        let data = if self.order == rhs.order {
            self.data.into_iter().zip(rhs.data.iter()).map(op).collect()
        } else {
            self.data
                .into_iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index) };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_consume_both<R, T, F>(
        self,
        rhs: Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<T>>
    where
        R: Clone,
        F: FnMut((L, R)) -> T,
    {
        ensure_elementwise_operation_conformable(&self, &rhs)?;

        let data = if self.order == rhs.order {
            self.data.into_iter().zip(rhs.data).map(op).collect()
        } else {
            self.data
                .into_iter()
                .enumerate()
                .map(|(index, left)| {
                    let index = translate_index_between_orders_unchecked(index, self.shape);
                    let right = unsafe { rhs.data.get_unchecked(index).clone() };
                    op((left, right))
                })
                .collect()
        };

        Ok(Matrix {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_assign<R, F>(&mut self, rhs: &Matrix<R>, mut op: F) -> Result<()>
    where
        F: FnMut((&mut L, &R)),
    {
        ensure_elementwise_operation_conformable(self, rhs)?;

        if self.order == rhs.order {
            self.data.iter_mut().zip(rhs.data.iter()).for_each(op);
        } else {
            self.data.iter_mut().enumerate().for_each(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, self.shape);
                let right = unsafe { rhs.data.get_unchecked(index) };
                op((left, right))
            });
        }

        Ok(())
    }

    pub fn elementwise_operation_assign_consume_rhs<R, F>(
        &mut self,
        rhs: Matrix<R>,
        mut op: F,
    ) -> Result<()>
    where
        R: Clone,
        F: FnMut((&mut L, R)),
    {
        ensure_elementwise_operation_conformable(self, &rhs)?;

        if self.order == rhs.order {
            self.data.iter_mut().zip(rhs.data).for_each(op);
        } else {
            self.data.iter_mut().enumerate().for_each(|(index, left)| {
                let index = translate_index_between_orders_unchecked(index, self.shape);
                let right = unsafe { rhs.data.get_unchecked(index).clone() };
                op((left, right))
            });
        }

        Ok(())
    }
}

impl<L> Matrix<L> {
    pub fn multiplication_like_operation<R, T, F>(
        &self,
        rhs: &Matrix<R>,
        mut op: F,
    ) -> Result<Matrix<T>>
    where
        L: std::ops::Mul<R, Output = T>,
        T: std::ops::Add<Output = T> + Default,
        F: FnMut(VectorIter<&L>, VectorIter<&R>) -> Option<T>,
    {
        ensure_multiplication_conformable(self, rhs)?;

        let nrows = self.nrows();
        let ncols = rhs.ncols();
        let size = nrows.checked_mul(ncols).ok_or(Error::SizeOverflow)?;

        let order = self.order;
        let shape = AxisShape::build((nrows, ncols), order)?;
        let mut data = Vec::with_capacity(size);
        match order {
            Order::RowMajor => {
                'outer: for row in 0..nrows {
                    for col in 0..ncols {
                        match op(self.iter_nth_row(row), rhs.iter_nth_col(col)) {
                            None => {
                                data.resize_with(size, T::default);
                                break 'outer;
                            }
                            Some(value) => data.push(value),
                        }
                    }
                }
            }
            Order::ColMajor => {
                'outer: for col in 0..ncols {
                    for row in 0..nrows {
                        match op(self.iter_nth_row(row), rhs.iter_nth_col(col)) {
                            None => {
                                data.resize_with(size, T::default);
                                break 'outer;
                            }
                            Some(value) => data.push(value),
                        }
                    }
                }
            }
        }

        Ok(Matrix { data, order, shape })
    }
}
