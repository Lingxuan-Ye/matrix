use super::index::Index;
use super::Matrix;
use crate::consts::{COMMA, LEFT_DELIMITER, RIGHT_DELIMITER, SEP_LEN, SPACE, TAB_LEN};

macro_rules! set_dim {
    ($($arg:tt)*) => {
        std::format_args!("\u{001b}[2m{}\u{001b}[22m", std::format_args!($($arg)*))
    };
}

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = self.size();
        let index_max_width = format!("{size}").chars().count();
        let mut element_max_width = 0;
        let mut cache = Vec::with_capacity(size);
        for element in self.data.iter() {
            let string = format!("{element:?}");
            let width = string.chars().count();
            if width > element_max_width {
                element_max_width = width;
            }
            cache.push(string);
        }

        writeln!(f, "Matrix{SPACE}{{")?;
        writeln!(f, "{SPACE:TAB_LEN$}data:")?;

        let shape = self.shape();
        write!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}")?;
        write!(f, "{LEFT_DELIMITER:<TAB_LEN$}")?;
        write!(f, "{SPACE:>index_max_width$}")?;
        write!(f, "{SPACE:SEP_LEN$}")?;
        write!(f, "{SPACE}")?;
        for col in 0..shape.ncols {
            if col != 0 {
                write!(f, "{SPACE:<SEP_LEN$}")?;
            }
            write!(f, "{}", set_dim!("{col:>index_max_width$}"))?;
            if col != (shape.ncols - 1) {
                write!(f, "{SPACE}")?;
                write!(f, "{SPACE:>element_max_width$}")?;
            }
        }
        writeln!(f)?;

        for row in 0..shape.nrows {
            write!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}")?;
            write!(f, "{SPACE:TAB_LEN$}")?;
            write!(f, "{}", set_dim!("{row:>index_max_width$}"))?;
            write!(f, "{SPACE:SEP_LEN$}")?;
            write!(f, "{LEFT_DELIMITER}")?;
            for col in 0..shape.ncols {
                if col != 0 {
                    write!(f, "{COMMA:<SEP_LEN$}")?;
                }
                let index = Index::new(row, col).into_flattened_unchecked_for(self);
                let element = &cache[index];
                write!(f, "{}", set_dim!("{index:>index_max_width$}"))?;
                write!(f, "{SPACE}")?;
                write!(f, "{element:>element_max_width$}")?;
            }
            writeln!(f, "{RIGHT_DELIMITER}{COMMA}")?;
        }

        writeln!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}{RIGHT_DELIMITER}")?;

        writeln!(f, "{SPACE:TAB_LEN$}order:{SPACE}{:?}", self.order)?;
        writeln!(f, "{SPACE:TAB_LEN$}shape:{SPACE}{:?}", self.shape)?;
        writeln!(f, "}}")
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let size = self.size();
        let mut element_max_width = 0;
        let mut cache = Vec::with_capacity(size);
        for element in self.data.iter() {
            let string = format!("{element}");
            let width = string.chars().count();
            if width > element_max_width {
                element_max_width = width;
            }
            cache.push(string);
        }

        let shape = self.shape();
        for row in 0..shape.nrows {
            if row == 0 {
                write!(f, "{LEFT_DELIMITER}{LEFT_DELIMITER}")?;
            } else {
                write!(f, "{SPACE}{LEFT_DELIMITER}")?;
            }
            for col in 0..shape.ncols {
                if col != 0 {
                    write! {f, "{SPACE:<SEP_LEN$}"}?;
                }
                let index = Index::new(row, col).into_flattened_unchecked_for(self);
                let element = &cache[index];
                write!(f, "{element:>element_max_width$}")?;
            }
            if row != (shape.nrows - 1) {
                writeln!(f, "{RIGHT_DELIMITER}")?;
            } else {
                writeln!(f, "{RIGHT_DELIMITER}{RIGHT_DELIMITER}")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix;

    #[test]
    fn test_debug() {
        let matrix = matrix![[1, 2, 3], [4, 5, 6]];
        let result = format!("{:?}", matrix);
        let expected = "\
Matrix {
    data:
        [       \u{1b}[2m0\u{1b}[22m    \u{1b}[2m1\u{1b}[22m    \u{1b}[2m2\u{1b}[22m
            \u{1b}[2m0\u{1b}[22m  [\u{1b}[2m0\u{1b}[22m 1, \u{1b}[2m1\u{1b}[22m 2, \u{1b}[2m2\u{1b}[22m 3],
            \u{1b}[2m1\u{1b}[22m  [\u{1b}[2m3\u{1b}[22m 4, \u{1b}[2m4\u{1b}[22m 5, \u{1b}[2m5\u{1b}[22m 6],
        ]
    order: RowMajor
    shape: AxisShape { major: 2, minor: 3 }
}
";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_display() {
        let matrix = matrix![[1, 2, 3], [4, 5, 6]];
        let result = format!("{}", matrix);
        let expected = "\
[[1  2  3]
 [4  5  6]]
";
        assert_eq!(result, expected);
    }
}
