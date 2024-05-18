use super::kind::Kind;
use super::Vector;
use crate::fmt::{
    COMMA, LEFT_DELIMITER, RIGHT_DELIMITER, SEP_LEN, SET_DIM, SPACE, TAB_LEN, UNSET_DIM,
};

impl<T: std::fmt::Debug> std::fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len();
        let index_max_width = format!("{len}").chars().count();
        let mut element_max_width = 0;
        let mut cache = Vec::with_capacity(len);
        for element in self.iter() {
            let string = format!("{element:?}");
            let width = string.chars().count();
            if width > element_max_width {
                element_max_width = width;
            }
            cache.push(string);
        }

        writeln!(f, "Vector{SPACE}{{")?;
        writeln!(f, "{SPACE:TAB_LEN$}data:")?;

        match self.kind {
            Kind::RowVector => {
                write!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}{LEFT_DELIMITER}")?;
                for (index, element) in cache.into_iter().enumerate() {
                    if index != 0 {
                        write!(f, "{COMMA:<SEP_LEN$}")?;
                    }
                    write!(f, "{SET_DIM}{index:>index_max_width$}{UNSET_DIM}")?;
                    write!(f, "{SPACE}")?;
                    write!(f, "{element:>element_max_width$}")?;
                }
                writeln!(f, "{RIGHT_DELIMITER}")?;
            }
            Kind::ColVector => {
                writeln!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}{LEFT_DELIMITER}")?;
                for (index, element) in cache.into_iter().enumerate() {
                    write!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}")?;
                    write!(f, "{SET_DIM}{index:>index_max_width$}{UNSET_DIM}")?;
                    write!(f, "{SPACE}")?;
                    write!(f, "{element:>element_max_width$}")?;
                    writeln!(f, "{COMMA}")?;
                }
                writeln!(f, "{SPACE:TAB_LEN$}{SPACE:TAB_LEN$}{RIGHT_DELIMITER}")?;
            }
        }

        writeln!(f, "{SPACE:TAB_LEN$}kind:{SPACE}{:?}", self.kind)?;
        writeln!(f, "}}")
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len();
        let mut element_max_width = 0;
        let mut cache = Vec::with_capacity(len);
        for element in self.iter() {
            let string = format!("{element:}");
            let width = string.chars().count();
            if width > element_max_width {
                element_max_width = width;
            }
            cache.push(string);
        }

        match self.kind {
            Kind::RowVector => {
                write!(f, "{LEFT_DELIMITER}")?;
                for (index, element) in cache.into_iter().enumerate() {
                    if index != 0 {
                        write!(f, "{SPACE:<SEP_LEN$}")?;
                    }
                    write!(f, "{element:>element_max_width$}")?;
                }
                writeln!(f, "{RIGHT_DELIMITER}")?;
            }
            Kind::ColVector => {
                for (index, element) in cache.into_iter().enumerate() {
                    if index == 0 {
                        write!(f, "{LEFT_DELIMITER}")?;
                    } else {
                        write!(f, "{SPACE}")?;
                    }
                    write!(f, "{element:>element_max_width$}")?;
                    if index != (len - 1) {
                        writeln!(f)?;
                    } else {
                        writeln!(f, "{RIGHT_DELIMITER}")?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::vector;

    #[test]
    fn test_debug() {
        let mut vector = vector![1, 2, 3];

        let result = format!("{:?}", vector);
        let expected = "\
Vector {
    data:
        [\u{1b}[2m0\u{1b}[22m 1, \u{1b}[2m1\u{1b}[22m 2, \u{1b}[2m2\u{1b}[22m 3]
    kind: RowVector
}
";
        assert_eq!(result, expected);

        vector.transpose();
        let result = format!("{:?}", vector);
        let expected = "\
Vector {
    data:
        [
            \u{1b}[2m0\u{1b}[22m 1,
            \u{1b}[2m1\u{1b}[22m 2,
            \u{1b}[2m2\u{1b}[22m 3,
        ]
    kind: ColVector
}
";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_display() {
        let mut vector = vector![1, 2, 3];

        let result = format!("{}", vector);
        let expected = "[1  2  3]\n";
        assert_eq!(result, expected);

        vector.transpose();
        let result = format!("{}", vector);
        let expected = "\
[1
 2
 3]
";
        assert_eq!(result, expected);
    }
}
