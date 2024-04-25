use super::index::Index;
use super::Matrix;

const LEFT_DELIMITER: &'static str = "[";
const RIGHT_DELIMITER: &'static str = "]";
const COMMA: &'static str = ",";
const SPACE: &'static str = " ";
const TAB: &'static str = "    ";
const SEP_LEN: usize = 2;

impl<T: std::fmt::Debug> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.size();
        let index_max_len = format!("{size}").chars().count();
        let mut element_max_len = 0;
        let mut cache = Vec::<String>::with_capacity(size);
        for element in self.data.iter() {
            let string = format!("{element:?}");
            let len = string.chars().count();
            if len > element_max_len {
                element_max_len = len;
            }
            cache.push(string);
        }

        writeln!(f, "Matrix{SPACE}{{")?;
        writeln!(f, "{TAB}data:")?;

        write!(f, "{TAB}{TAB}{LEFT_DELIMITER:<index_max_len$}{TAB}{SPACE}")?;
        for col in 0..self.ncols() {
            if col != 0 {
                write! {f, "{SPACE:<SEP_LEN$}"}?;
            }
            write!(
                f,
                "{col:>index_max_len$}{SPACE:<SEP_LEN$}{SPACE:>element_max_len$}"
            )?;
        }
        writeln!(f)?;

        for row in 0..self.nrows() {
            write!(f, "{TAB}{TAB}{row:>index_max_len$}{TAB}{LEFT_DELIMITER}")?;
            for col in 0..self.ncols() {
                if col != 0 {
                    write! {f, "{COMMA:<SEP_LEN$}"}?;
                }
                let index = self.flatten_index(Index::new(row, col));
                let element = &cache[index];
                write!(
                    f,
                    "{index:>index_max_len$}{SPACE:<SEP_LEN$}{element:>element_max_len$}"
                )?;
            }
            writeln!(f, "{RIGHT_DELIMITER}{COMMA}")?;
        }

        writeln!(f, "{TAB}{TAB}{RIGHT_DELIMITER}")?;
        writeln!(f)?;
        writeln!(f, "{TAB}order:{SPACE}{:?}", self.order)?;
        writeln!(f)?;
        writeln!(f, "{TAB}shape:{SPACE}{:?}", self.shape)?;
        writeln!(f, "}}")
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.size();
        let mut element_max_len = 0;
        let mut cache = Vec::<String>::with_capacity(size);
        for element in self.data.iter() {
            let string = format!("{element}");
            let len = string.chars().count();
            if len > element_max_len {
                element_max_len = len;
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
                let index = self.flatten_index(Index::new(row, col));
                let element = &cache[index];
                write!(f, "{element:>element_max_len$}")?;
            }
            writeln!(f, "{RIGHT_DELIMITER}{COMMA}")?;
        }

        writeln!(f, "{RIGHT_DELIMITER}")
    }
}
