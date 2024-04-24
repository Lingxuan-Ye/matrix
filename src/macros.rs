#[macro_export]
macro_rules! matrix {
    [ $($col:expr),+ $(,)? ] => {
        $crate::matrix::Matrix::from_2darray(std::boxed::Box::new([$($col,)+]))
    };
}

#[macro_export]
macro_rules! shape {
    ( $nrows:expr, $ncols:expr ) => {
        match $crate::matrix::shape::Shape::build($nrows, $ncols) {
            Ok(shape) => shape,
            Err(error) => panic!("{error}"),
        }
    };
}
