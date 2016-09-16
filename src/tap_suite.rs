use tap_test::TapTest;

/// Represents a collection of TAP tests (TapTest) which can be rendered into a (text) TAP stream. This orchestrates that rendering.
#[derive(Debug)]
pub struct TapSuite {
    pub tests: Vec<TapTest>,
}

impl TapSuite {
    /// Produce and arrange all text lines, in order, included in this TAP stream.
    pub fn lines(&self) -> Vec<String> {
        let first_line = format!("1..{}", self.tests.len());
        let mut all_lines = vec![first_line];

        for (i, test) in self.tests.iter().enumerate() {
            let index = i as i64;
            let tap = test.tap(index + 1);
            all_lines.extend(tap.iter().cloned());
        }

        all_lines
    }

    /// Emit TAP stream to standard output.
    pub fn print(&self) {
        for line in self.lines() {
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TapSuite;
    use tap_test_builder::TapTestBuilder;
    
    #[test]
    fn test_lines() {
        let passing_test = TapTestBuilder::new()
            .name("Panda Bamboo")
            .passed(true)
            .finalize();

        let failing_test = TapTestBuilder::new()
            .name("Curry Noodle")
            .passed(false)
            .commentary(vec!["Tree".to_string(), "Flower".to_string()])
            .finalize();

        let tap_suite = TapSuite {
            tests: vec![passing_test, failing_test],
        };

        let expected = vec![
            "1..2",
            "ok 1 Panda Bamboo",
            "not ok 2 Curry Noodle",
            "# Tree",
            "# Flower"
        ];
        let actual = tap_suite.lines();
        
        assert_eq!(expected, actual);

    }
}
