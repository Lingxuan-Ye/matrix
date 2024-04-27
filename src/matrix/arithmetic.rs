mod add;
mod sub;

use super::index::AxisIndex;
use super::Matrix;
use crate::error::{Error, Result};

impl<T> Matrix<T> {
    pub fn ensure_same_shape<U>(&self, other: &Matrix<U>) -> Result<()> {
        if self.shape() != other.shape() {
            Err(Error::MatricesInconformable)
        } else {
            Ok(())
        }
    }

    pub fn ensure_multiplication_conformable<U>(&self, other: &Matrix<U>) -> Result<()> {
        if self.ncols() != other.nrows() {
            Err(Error::MatricesInconformable)
        } else {
            Ok(())
        }
    }

    pub fn elementwise_operation<U, V, F>(&self, rhs: &Matrix<U>, mut op: F) -> Result<Matrix<V>>
    where
        F: FnMut((&T, &U)) -> V,
    {
        self.ensure_same_shape(rhs)?;

        let data = if self.order == rhs.order {
            self.data.iter().zip(rhs.data.iter()).map(op).collect()
        } else {
            self.data
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    let index = AxisIndex::from_flattened_unchecked(index, self.shape)
                        .interpret_with(self.order);
                    op((element, &rhs[index]))
                })
                .collect()
        };

        Ok(Matrix::<V> {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_consume_rhs<U, V, F>(
        &self,
        mut rhs: Matrix<U>,
        op: F,
    ) -> Result<Matrix<V>>
    where
        F: FnMut((&T, U)) -> V,
    {
        self.ensure_same_shape(&rhs)?;

        if self.order != rhs.order {
            rhs.switch_order();
        }

        let data = self.data.iter().zip(rhs.data.into_iter()).map(op).collect();

        Ok(Matrix::<V> {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_consume_self<U, V, F>(
        self,
        rhs: &Matrix<U>,
        mut op: F,
    ) -> Result<Matrix<V>>
    where
        F: FnMut((T, &U)) -> V,
    {
        self.ensure_same_shape(rhs)?;

        let data = if self.order == rhs.order {
            self.data.into_iter().zip(rhs.data.iter()).map(op).collect()
        } else {
            self.data
                .into_iter()
                .enumerate()
                .map(|(index, element)| {
                    let index = AxisIndex::from_flattened_unchecked(index, self.shape)
                        .interpret_with(self.order);
                    op((element, &rhs[index]))
                })
                .collect()
        };

        Ok(Matrix::<V> {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_consume_both<U, V, F>(
        self,
        mut rhs: Matrix<U>,
        op: F,
    ) -> Result<Matrix<V>>
    where
        F: FnMut((T, U)) -> V,
    {
        self.ensure_same_shape(&rhs)?;

        if self.order != rhs.order {
            rhs.switch_order();
        }

        let data = self
            .data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(op)
            .collect();

        Ok(Matrix::<V> {
            data,
            order: self.order,
            shape: self.shape,
        })
    }

    pub fn elementwise_operation_assign<U, F>(&mut self, rhs: &Matrix<U>, mut op: F) -> Result<()>
    where
        F: FnMut((&mut T, &U)),
    {
        self.ensure_same_shape(rhs)?;

        if self.order == rhs.order {
            self.data.iter_mut().zip(rhs.data.iter()).for_each(op);
        } else {
            self.data
                .iter_mut()
                .enumerate()
                .for_each(|(index, element)| {
                    let index = AxisIndex::from_flattened_unchecked(index, self.shape)
                        .interpret_with(self.order);
                    op((element, &rhs[index]))
                });
        }

        Ok(())
    }

    pub fn elementwise_operation_assign_consume_rhs<U, F>(
        &mut self,
        mut rhs: Matrix<U>,
        op: F,
    ) -> Result<()>
    where
        F: FnMut((&mut T, U)),
    {
        self.ensure_same_shape(&rhs)?;

        if self.order != rhs.order {
            rhs.switch_order();
        }

        self.data.iter_mut().zip(rhs.data.into_iter()).for_each(op);

        Ok(())
    }
}
