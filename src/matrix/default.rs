use super::order::Order;
use super::shape::AxisShape;
use super::Matrix;

impl<T> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            order: Order::default(),
            shape: AxisShape::default(),
            data: Vec::default(),
        }
    }
}
