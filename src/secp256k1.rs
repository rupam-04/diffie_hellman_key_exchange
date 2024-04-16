use crate::u256::RD256;
use std::str::FromStr;

#[derive(Debug, Clone)]

pub struct Point {
    pub x: RD256,
    pub y: RD256
}

impl Point {
    pub fn from_hex_coordinates(x: &str, y: &str) -> Self {
        return Point {
            x: RD256::from_str(x).unwrap(),
            y: RD256::from_str(y).unwrap()
        };
    }
    pub fn to_hex_string(&self) -> String {
        return format!("04{}{}", self.x.to_string(), self.y.to_string());
    }
    pub fn is_zero_point(&self) -> bool {
        return self.x == RD256::from_str("0x0").unwrap() && self.y == RD256::from_str("0x0").unwrap();
    }
}

pub struct SECP256K1;

impl SECP256K1 {
    pub fn p() -> RD256 {
        return RD256::from_str("0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f").unwrap();
    }
    pub fn g() -> Point {
        return Point {
            x: RD256::from_str("0x79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798").unwrap(),
            y: RD256::from_str("0x483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8").unwrap()
        };
    }

    pub fn add_points(pt1: &Point, pt2: &Point) -> Point {
        println!("Adding");
        assert!(pt1.y != pt2.y);

        if pt1.is_zero_point() {
            return pt2.clone();
        }
        if pt2.is_zero_point() {
            return pt1.clone();
        }

        let p: &RD256 = &Self::p();


        //slope calculation
        let y_diff: &RD256 = &pt1.y.sub_mod(&pt2.y, p);
        let x_diff: &RD256 = &pt1.x.sub_mod(&pt2.x, p);
        let lambda: &RD256 = &y_diff.div_mod(&x_diff, p);

        //x3 calculation
        let x3: &RD256 = &lambda.mul_mod(lambda, p).sub_mod(&pt1.x, p).sub_mod(&pt2.x, p);

        //y3 calculation
        let y3: &RD256 = &pt1.x.sub_mod(x3, p).mul_mod(lambda, p).sub_mod(&pt1.y, p);

        return Point {
            x: x3.clone(),
            y: y3.clone()
        };
    }

    pub fn double_point(pt: &Point) -> Point {
        println!("Doubling");
        if pt.is_zero_point() {
            return pt.clone();
        }
        if pt.y == RD256::from_str("0x0").unwrap() {
            return Self::zero_point().clone();
        }

        let p: &RD256 = &Self::p();
        let const_2: &RD256 = &RD256::from_str("0x2").unwrap();
        let const_3: &RD256 = &RD256::from_str("0x3").unwrap();

        //slope calculation
        let two_y: &RD256 = &pt.y.mul_mod(const_2, p);
        let x1_2_3: &RD256 = &pt.x.mul_mod(&pt.x, p).mul_mod(const_3, p);
        let lambda: &RD256 = &x1_2_3.div_mod(&two_y, p);

        //x3 calculation
        let x3: &RD256 = &lambda.mul_mod(lambda, p).sub_mod(&pt.x, p).sub_mod(&pt.x, p);

        //y3 calculation
        let y3: &RD256 = &pt.x.sub_mod(x3, p).mul_mod(lambda, p).sub_mod(&pt.y, p);

        return Point {
            x: x3.clone(),
            y: y3.clone()
        };
    }

    pub fn zero_point() -> Point {
        return Point {
            x: RD256::from_str("0x0").unwrap(),
            y: RD256::from_str("0x0").unwrap()
        };
    }

    pub fn mul_point(pt: &Point, n: &RD256) -> Point {
        let mut result: Point = Self::zero_point();
        let mut addend: Point = pt.clone();

        let mut n_copy: RD256 = n.clone();

        while n_copy != RD256::from_str("0x0").unwrap() {
            if n_copy.is_odd() {
                result = Self::add_points(&result, &addend);
            }
            addend = Self::double_point(&addend);
            n_copy = n_copy.div_mod(&RD256::from_str("0x2").unwrap(), &Self::p());
        }

        return result;
    }
}