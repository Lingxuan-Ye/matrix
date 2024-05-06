use super::Matrix;

const LEFT_DELIMITER: &str = "[";
const RIGHT_DELIMITER: &str = "]";
const COMMA: &str = ",";
const SPACE: &str = " ";
const TAB: &str = "    ";
const SET_DIM: &str = "\u{001b}[2m";
const UNSET_DIM: &str = "\u{001b}[22m";
const SEP_LEN: usize = 2;

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        writeln!(f, "{TAB}data:")?;

        write!(f, "{TAB}{TAB}")?;
        write!(f, "{LEFT_DELIMITER}")?;
        write!(f, "{SPACE:<index_max_width$}")?;
        write!(f, "{TAB}")?;
        write!(f, "{SPACE}")?;
        for col in 0..self.ncols() {
            if col != 0 {
                write! {f, "{SPACE:<SEP_LEN$}"}?;
            }
            write!(f, "{SET_DIM}{col:>index_max_width$}{UNSET_DIM}")?;
            write!(f, "{SPACE}")?;
            write!(f, "{SPACE:>element_max_width$}")?;
        }
        writeln!(f)?;

        for row in 0..self.nrows() {
            write!(f, "{TAB}{TAB}")?;
            write!(f, "{SPACE}")?;
            write!(f, "{SET_DIM}{row:>index_max_width$}{UNSET_DIM}")?;
            write!(f, "{TAB}")?;
            write!(f, "{LEFT_DELIMITER}")?;
            for col in 0..self.ncols() {
                if col != 0 {
                    write! {f, "{COMMA:<SEP_LEN$}"}?;
                }
                let index = self.flatten_index_unchecked((row, col));
                let element = &cache[index];
                write!(f, "{SET_DIM}{index:>index_max_width$}{UNSET_DIM}")?;
                write!(f, "{SPACE}")?;
                write!(f, "{element:>element_max_width$}")?;
            }
            writeln!(f, "{RIGHT_DELIMITER}{COMMA}")?;
        }

        writeln!(f, "{TAB}{TAB}{RIGHT_DELIMITER}")?;

        writeln!(f, "{TAB}order:{SPACE}{:?}", self.order)?;
        writeln!(f, "{TAB}shape:{SPACE}{:?}", self.shape)?;
        writeln!(f, "}}")
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        writeln!(f, "{LEFT_DELIMITER}")?;

        for row in 0..self.nrows() {
            write!(f, "{TAB}{LEFT_DELIMITER}")?;
            for col in 0..self.ncols() {
                if col != 0 {
                    write! {f, "{COMMA:<SEP_LEN$}"}?;
                }
                let index = self.flatten_index_unchecked((row, col));
                let element = &cache[index];
                write!(f, "{element:>element_max_width$}")?;
            }
            writeln!(f, "{RIGHT_DELIMITER}{COMMA}")?;
        }

        writeln!(f, "{RIGHT_DELIMITER}")
    }
}
