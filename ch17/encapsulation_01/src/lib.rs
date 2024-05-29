pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
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
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_one_element_should_work() {
        let mut obj = AveragedCollection{
            list: vec![], 
            average: 0.0,
        };
        obj.add(1);

        assert_eq!(obj.average(), 1.0);
    }

    #[test]
    fn add_a_few_elements_should_work() {
        let mut obj = AveragedCollection{
            list: vec![], 
            average: 0.0,
        };
        obj.add(1);
        obj.add(2);
        obj.add(3);

        assert_eq!(obj.average(), 2.0);
    }

    #[test]
    fn remove_elements_should_work() {
        let mut obj = AveragedCollection{
            list: vec![], 
            average: 0.0,
        };
        obj.add(1);
        obj.add(2);
        obj.add(3);
        obj.remove();

        assert_eq!(obj.average(), 1.5);
    }
}