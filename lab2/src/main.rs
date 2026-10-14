#[derive(Debug, Clone, Default)]
struct NumberWithUnit{
    unit: String,
    value: f64
}
impl NumberWithUnit {
    fn add(self, other: NumberWithUnit) -> NumberWithUnit {
        if self.unit != other.unit {
            panic!();
        }
        NumberWithUnit{
            unit: self.unit,
            value: self.value + other.value,
        }
    }
    fn mul(self, other: NumberWithUnit) -> NumberWithUnit {
        NumberWithUnit{
            unit: format!("{}*{}", self.unit, other.unit),
            value: self.value * other.value,
        }
    }
    fn div(self, other: NumberWithUnit) -> NumberWithUnit {
        NumberWithUnit{
            unit: format!("{}/{}", self.unit, other.unit),
            value: self.value / other.value,
        }
    }
    fn add_in_place(&mut self, other: NumberWithUnit) -> NumberWithUnit {
        if self.unit != other.unit {
            panic!();
        }
        self.value+=other.value;
        self.clone()
    }
    fn mul_in_place(&mut self, other: NumberWithUnit) -> NumberWithUnit {
        self.unit=format!("{}*{}", self.unit, other.unit);
        self.value*=other.value;
        self.clone()
    }
    fn div_in_place(&mut self, other: NumberWithUnit) -> NumberWithUnit {
        self.unit=format!("{}/{}", self.unit, other.unit);
        self.value/=other.value;
        self.clone()
    }
}

fn unitless(value: f64) -> NumberWithUnit {
    NumberWithUnit {
        value,
        ..Default::default()
    }
}

fn with_unit(value: f64, unit: &str) -> NumberWithUnit {
    NumberWithUnit {
        unit: String::from(unit),
        value
    }
}

fn with_unit_from(other: &NumberWithUnit, value: f64) -> NumberWithUnit {
    NumberWithUnit {
        unit: other.unit.clone(),
        value
    }
}

fn main() {
    let x = unitless(42.0);
    let y = with_unit(3.14, "m");
    let z = with_unit_from(&y, 2.71);
    
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
