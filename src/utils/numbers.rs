// Finds what the closest number the current value is to,
// for a given series of steps.
// The steps should be in ascending order.
// This can be used for incremental updates that are not overly granular.
pub fn get_range_increment<T>(steps: &Vec<T>, value: T) -> T
where
    T: PartialOrd + Copy,
{
    let mut closest = &steps[0];
    for step in steps {
        if step > &value {
            break;
        }
        closest = step;
    }
    *closest
}
