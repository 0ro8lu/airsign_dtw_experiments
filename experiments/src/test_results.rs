use std::fmt;

#[derive(Clone, Copy)]
pub struct TestResult {
    pub false_accept: u32,
    pub false_reject: u32,
    pub genuine_accept: u32,
    pub genuine_reject: u32,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "false_accept: {}, false_reject: {}, genuine_accept: {}, genuine_reject: {}",
            self.false_accept, self.false_reject, self.genuine_accept, self.genuine_reject
        )
    }
}

impl TestResult {
    pub fn new() -> Self {
        TestResult {
            false_accept: 0,
            false_reject: 0,
            genuine_accept: 0,
            genuine_reject: 0,
        }
    }
}
