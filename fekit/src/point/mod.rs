/**
 * Author: V. Ganesh
 * License: MIT
 */

/** point module, Point struct and methods */

/** Point - represents a point in space */
#[allow(dead_code)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Point {
    pub fn add(&mut self, v: f32) -> Point {
        return Point {
            x: self.x + v,
            y: self.y + v,
            z: self.z + v,
        };
    }

    pub fn sub(&mut self, v: f32) -> Point {
        return Point {
            x: self.x - v,
            y: self.y - v,
            z: self.z - v,
        };
    }

    pub fn mul(&mut self, v: f32) -> Point {
        return Point {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        };
    }

    pub fn div(&mut self, v: f32) -> Point {
        return Point {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v,
        };
    }

    pub fn add_point(&mut self, pt: &Point) -> Point {
        return Point {
            x: self.x + pt.x,
            y: self.y + pt.y,
            z: self.z + pt.z,
        };
    }

    pub fn sub_point(&mut self, pt: &Point) -> Point {
        return Point {
            x: self.x - pt.x,
            y: self.y - pt.y,
            z: self.z - pt.z,
        };
    }

    pub fn mul_point(&mut self, pt: &Point) -> Point {
        return Point {
            x: self.x * pt.x,
            y: self.y * pt.y,
            z: self.z * pt.z,
        };
    }

    pub fn sqr_point(&mut self) -> Point {
        return Point {
            x: self.x * self.x,
            y: self.y * self.y,
            z: self.z * self.z,
        };
    }

    pub fn div_point(&mut self, pt: &Point) -> Point {
        return Point {
            x: self.x / pt.x,
            y: self.y / pt.y,
            z: self.z / pt.z,
        };
    }

    pub fn distance_from(&mut self, pt: &Point) -> f32 {
        let new_pt = self.sub_point(pt).sqr_point();

        return (new_pt.x + new_pt.y + new_pt.z).sqrt();
    }
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    #[test]
    fn point_init() {
        let point = super::Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }

    #[test]
    fn point_sclar_fn() {
        let mut pt1 = super::Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = 1.0;

        let add_pt = pt1.add(v1);
        assert_eq!(add_pt.x, 1.0);
        assert_eq!(add_pt.y, 1.0);
        assert_eq!(add_pt.z, 1.0);

        let sub_pt = pt1.sub(v1);
        assert_eq!(sub_pt.x, -1.0);
        assert_eq!(sub_pt.y, -1.0);
        assert_eq!(sub_pt.z, -1.0);

        let mut pt2 = super::Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        };
        let v2 = 2.0;

        let mul_pt = pt2.mul(v2);
        assert_eq!(mul_pt.x, 2.0);
        assert_eq!(mul_pt.y, 0.0);
        assert_eq!(mul_pt.z, -2.0);

        let div_pt = pt2.div(v2);
        assert_eq!(div_pt.x, 0.5);
        assert_eq!(div_pt.y, 0.0);
        assert_eq!(div_pt.z, -0.5);
    }

    #[test]
    fn point_vec_fn() {
        let mut pt1 = super::Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let v1 = super::Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let add_pt = pt1.add_point(&v1);
        assert_eq!(add_pt.x, 1.0);
        assert_eq!(add_pt.y, 1.0);
        assert_eq!(add_pt.z, 1.0);

        let sub_pt = pt1.sub_point(&v1);
        assert_eq!(sub_pt.x, -1.0);
        assert_eq!(sub_pt.y, -1.0);
        assert_eq!(sub_pt.z, -1.0);

        let mut pt2 = super::Point {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        };
        let v2 = super::Point {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        let mul_pt = pt2.mul_point(&v2);
        assert_eq!(mul_pt.x, 2.0);
        assert_eq!(mul_pt.y, 0.0);
        assert_eq!(mul_pt.z, -2.0);

        let div_pt = pt2.div_point(&v2);
        assert_eq!(div_pt.x, 0.5);
        assert_eq!(div_pt.y, 0.0);
        assert_eq!(div_pt.z, -0.5);
    }

    #[test]
    fn point_dist_fn() {
        let mut pt1 = super::Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut pt2 = super::Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let sq_pt = pt2.sqr_point();
        assert_eq!(sq_pt.x, 1.0);
        assert_eq!(sq_pt.y, 0.0);
        assert_eq!(sq_pt.z, 0.0);

        let dist = pt1.distance_from(&pt2);
        assert_eq!(dist, 1.0);
    }
}
