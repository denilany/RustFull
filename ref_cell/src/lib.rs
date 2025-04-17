mod messenger;
pub use messenger::*;

pub use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(s: usize) -> Worker {
        Worker {
            track_value: Rc::new(s),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(vec![]),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }

    fn info(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }

    fn error(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        #[derive(Clone, Debug)]
        struct TestMs {
            value: Rc<usize>,
            ms: RefCell<Vec<String>>,
            correct: Vec<String>,
        }

        impl Logger for TestMs {
            fn warning(&self, message: &str) {
                self.ms.borrow_mut().push(message.to_string());
            }
            fn info(&self, message: &str) {
                self.ms.borrow_mut().push(message.to_string());
            }
            fn error(&self, message: &str) {
                self.ms.borrow_mut().push(message.to_string());
            }
        }

        let l = TestMs {
            value: Rc::new(115),
            ms: RefCell::new(vec![]),
            correct: vec![
                String::from("Info: you are using up to 40% of your quota"),
                String::from(
                    "Warning: you have used up over 80% of your quota! Proceeds with precaution",
                ),
                String::from("Error: you are over your quota!"),
            ],
        };

        let track = Tracker::new(&l, 5);
        let _a = l.value.clone();
        track.peek(&l.value);
        let _a1 = l.value.clone();
        let _a2 = l.value.clone();
        track.set_value(&l.value);
        let _a3 = l.value.clone();
        track.set_value(&l.value);

        for (i, v) in l.ms.into_inner().iter().enumerate() {
            assert_eq!(v, &l.correct[i])
        }
    }

    #[test]
    fn test_module_usage_hasmap() {
        let log = Worker::new(1000);
        let track = Tracker::new(&log, 12);
        let _clone_test = log.track_value.clone();
        let _clone_test1 = log.track_value.clone();
        let _clone_test2 = log.track_value.clone();
        let _clone_test3 = log.track_value.clone();
        let _clone_test4 = log.track_value.clone();
        let _clone_test5 = log.track_value.clone();
        let _clone_test6 = log.track_value.clone();
        let _clone_test7 = log.track_value.clone();

        track.set_value(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Warning").unwrap(),
            "you have used up over 75% of your quota! Proceeds with precaution"
        );

        let _clone_test8 = log.track_value.clone();

        track.set_value(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Warning").unwrap(),
            "you have used up over 83% of your quota! Proceeds with precaution"
        );

        track.peek(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Info").unwrap(),
            "you are using up to 83% of your quota"
        );

        let _clone_test9 = log.track_value.clone();

        track.peek(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Info").unwrap(),
            "you are using up to 91% of your quota"
        );

        let _clone_test10 = log.track_value.clone();

        track.set_value(&log.track_value);
        assert_eq!(
            log.mapped_messages.borrow().get("Error").unwrap(),
            "you are over your quota!"
        );
    }

    #[test]
    fn test_module_usage_vector() {
        let correct = vec![
            "Info: you are using up to 40% of your quota",
            "Warning: you have used up over 80% of your quota! Proceeds with precaution",
            "Info: you are using up to 80% of your quota",
            "Error: you are over your quota!",
        ];
        let log = Worker::new(1);
        let track = Tracker::new(&log, 5);
        let _a = log.track_value.clone();

        track.peek(&log.track_value);
        let _a1 = log.track_value.clone();
        let _a2 = log.track_value.clone();

        track.set_value(&log.track_value);

        track.peek(&log.track_value);
        let _a3 = log.track_value.clone();


        track.set_value(&log.track_value);

        for (i, v) in log.all_messages.into_inner().iter().enumerate() {
            assert_eq!(v, correct[i]);
        }
    }
}