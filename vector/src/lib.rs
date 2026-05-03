#[derive(Debug, PartialEq, Eq, Default)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Vector { data: value }
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
}
