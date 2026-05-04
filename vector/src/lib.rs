use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Vector { data: value }
    }
}

impl<T, Rhs> Add<Rhs> for Vector<T>
where
    T: Add<Rhs>,
    Rhs: Copy,
{
    type Output = Vector<<T as Add<Rhs>>::Output>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Vector {
            data: self.data.into_iter().map(|value| value + rhs).collect(),
        }
    }
}

impl<'a, T, Rhs> Add<Rhs> for &'a Vector<T>
where
    &'a T: Add<Rhs>,
    Rhs: Copy,
{
    type Output = Vector<<&'a T as Add<Rhs>>::Output>;

    fn add(self, rhs: Rhs) -> Self::Output {
        Vector {
            data: self.data.iter().map(|value| value + rhs).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_empty_vector() {
        let result: Vector<bool> = Vector::<bool>::default();
        assert_eq!(result, Vector { data: vec![] });
    }

    #[test]
    fn create_vector_from_vec() {
        let result = Vector::from(vec![1]);
        assert_eq!(result, Vector { data: vec![1] });
    }

    #[test]
    fn transform_vec_into_vector() {
        let result: Vector<i32> = vec![1].into();
        assert_eq!(result, Vector { data: vec![1] });
    }

    #[test]
    fn add_to_ref_of_vector() {
        let result = &Vector::from(vec![1, 2]) + 1;
        assert_eq!(result, Vector::from(vec![2, 3]))
    }

    #[test]
    fn add_to_vector() {
        let result = Vector::from(vec![1, 2]) + 1;
        assert_eq!(result, Vector::from(vec![2, 3]))
    }

    #[test]
    fn add_ref_to_ref_vector() {
        let result = &Vector::from(vec![1, 2]) + &1;
        assert_eq!(result, Vector::from(vec![2, 3]))
    }

    #[test]
    fn add_ref_to_vector() {
        let result = Vector::from(vec![1, 2]) + &1;
        assert_eq!(result, Vector::from(vec![2, 3]))
    }

    #[test]
    fn add_str_to_vector_of_strings() {
        let actual = Vector::from(vec!["Hello".to_string(), "Goodbye".to_string()]) + " world!";
        assert_eq!(
            Vector::from(vec![
                "Hello world!".to_string(),
                "Goodbye world!".to_string()
            ]),
            actual
        )
    }
}
