// structs can be viewed as being similar to objects (depending on your definition)
pub struct AveragedCollection {
    // fields in structs are private by default unless specified otherwise with `pub`
    // this enables encapsulation
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn from(list: Vec<i32>) -> AveragedCollection {
        let mut result = AveragedCollection {
            list,
            average: 0.0,
        };

        result.update_average();

        result
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();

        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None
        }
    }

    // private field accessor
    pub fn average(&self) -> f64 {
        self.average
    }

    // private method
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add() {
        let mut avg_col = AveragedCollection::new();

        avg_col.add(10);

        assert_eq!(avg_col.list, vec![10]);
        assert_eq!(avg_col.average, 10.0);
    }

    #[test]
    fn it_can_remove() {
        let mut avg_col = AveragedCollection::from(
            vec![10, 2, 1]
        );

        avg_col.remove();

        assert_eq!(avg_col.list, vec![10, 2]);
        assert_eq!(avg_col.average, 6.0);
    }

    #[test]
    fn it_can_average() {
        let avg_col = AveragedCollection::from(
            vec![1, 3, 5]
        );

        assert_eq!(avg_col.average, 3.0);
    }
}
