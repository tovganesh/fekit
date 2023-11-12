/** point module, Point struct and methods */

/** Point - represents a point in space */
#[allow(dead_code)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    #[test]
    fn point_init() {
        let point = super::Point { x: 0.0, y: 0.0, z: 0.0 };

        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }
}
