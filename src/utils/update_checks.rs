use std::collections::HashMap;

/// Update tracker for performing an action every n iterations.
/// This tracker has a changable target.
/// Each iteration you can iterate and check.
struct OcassionalUpdateTracker {
    /// The update target value for the iteration count.
    pub target: i32,
    /// The current iteration count.
    pub iteration: i32,
}

impl OcassionalUpdateTracker {
    /// Create an update tracker from a target iteration value.
    pub fn with_target(target: i32) -> Self {
        Self {
            target,
            ..Default::default()
        }
    }
}

impl Default for OcassionalUpdateTracker {
    fn default() -> Self {
        Self {
            target: 10,
            iteration: 0,
        }
    }
}

impl OcassionalUpdateTracker {
    /// Is this iteration the target iteration?
    pub fn check(&self) -> bool {
        self.iteration == 0
    }

    /// Iterate the tracker, restarting when target reached.
    pub fn iterate(&mut self) {
        self.iteration = (self.iteration + 1) % self.target;
    }

    /// Iterate the tracker and return whether iteration is the target.
    pub fn iterate_and_check(&mut self) -> bool {
        let ready = self.check();
        self.iterate();
        ready
    }
}

/// Records a value for a diff comparison at a later point in time.
#[derive(Default)]
pub struct ComparisonBody<T: PartialEq + Copy>(T);

impl<T: PartialEq + Copy> ComparisonBody<T> {
    /// Update the stored value that is being used for comparison.
    pub fn update_value(&mut self, v: T) {
        if self.0 != v {
            self.0 = v;
        }
    }

    /// Compare a value with the stored value.
    pub fn compare_to(&self, new_value: &T) -> bool {
        &self.0 == new_value
    }

    /// Compare values and store the new value.
    pub fn compare_and_update(&mut self, new_value: &T) -> bool {
        let equal = self.compare_to(new_value);
        if !equal {
            self.update_value(*new_value);
        }
        equal
    }
}

#[test]
fn test_comparison_body() {
    let mut prior_value = ComparisonBody("912760ndf2503");
    assert!(
        prior_value.compare_to(&"912760ndf2503"),
        "Compared values should be same"
    );
    assert!(
        prior_value.compare_to(&"97235jhasdfD") == false,
        "Different value is not the same"
    );
    assert!(
        prior_value.compare_and_update(&"L:Jyohasdofh29735") == false,
        "Comparing and updating with new value should be false (not same)"
    );
    assert!(
        prior_value.compare_and_update(&"L:Jyohasdofh29735"),
        "Comparing and updating with same value again should be true"
    );
    assert!(
        prior_value.0 == "L:Jyohasdofh29735",
        "The value should be the value that has been set"
    );
    prior_value.update_value(&"LAHFLHADF)72795");
    assert!(
        prior_value.compare_to(&"LAHFLHADF)72795"),
        "After calling update the value should be the value specified"
    );
}

pub type Comparisons<T: PartialEq + Copy> = HashMap<String, ComparisonBody<T>>;
