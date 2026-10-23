#[derive(Debug, Clone, Default)]
struct NumberWithUnit {
    unit: String,
    value: f64,
}

impl NumberWithUnit {
    fn unitless(value: f64) -> Self {
        Self {
            unit: String::new(),
            value,
        }
    }

    fn with_unit(value: f64, unit: String) -> Self {
        Self { unit, value }
    }

    fn with_unit_from(other: Self, value: f64) -> Self {
        Self {
            unit: other.unit,
            value,
        }
    }

    fn add(self, other: Self) -> Self {
        if self.unit != other.unit {
            panic!();
        }
        Self {
            unit: self.unit,
            value: self.value + other.value,
        }
    }

    fn mul(self, other: Self) -> Self {
        Self {
            unit: format!("{}*{}", self.unit, other.unit),
            value: self.value * other.value,
        }
    }

    fn div(self, other: Self) -> Self {
        Self {
            unit: format!("{}/{}", self.unit, other.unit),
            value: self.value / other.value,
        }
    }

    fn add_in_place(&mut self, other: &Self) {
        if self.unit != other.unit {
            panic!();
        }
        self.value += other.value;
    }

    fn mul_in_place(&mut self, other: &Self) {
        self.unit = format!("{}*{}", self.unit, other.unit);
        self.value *= other.value;
    }

    fn div_in_place(&mut self, other: &Self) {
        self.unit = format!("{}/{}", self.unit, other.unit);
        self.value /= other.value;
    }
}

fn mul_vals(values: &[NumberWithUnit]) -> NumberWithUnit {
    let mut result = values[0].clone();
    for value in &values[1..] {
        result.mul_in_place(value);
    }
    result
}

fn mul_vals_vec(values: Vec<NumberWithUnit>) -> NumberWithUnit {
    let mut result = values[0].clone();
    for value in &values[1..] {
        result.mul_in_place(value);
    }
    result
}

struct DoubleString(String, String);

impl DoubleString {
    fn from_strs(str_1: &str, str_2: &str) -> Self {
        Self(String::from(str_1), String::from(str_2))
    }

    fn from_strings(str_1: &String, str_2: &String) -> Self {
        Self(str_1.clone(), str_2.clone())
    }

    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}

fn main() {
    let a = NumberWithUnit::unitless(2.0);
    let b = NumberWithUnit::with_unit(100.0, String::from("m"));
    let c = NumberWithUnit::with_unit_from(b.clone(), 20.0);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    let time = NumberWithUnit::with_unit(5.0, String::from("s"));
    println!("{:?}", b.clone().add(c.clone()));
    println!("{:?}", b.clone().mul(time.clone()));
    println!("{:?}", b.clone().div(time.clone()));

    let mut result = b.clone();
    result.add_in_place(&c);
    println!("{:?}", result);

    let mut result = b.clone();
    result.mul_in_place(&time);
    println!("{:?}", result);

    let mut result = b.clone();
    result.div_in_place(&time);
    println!("{:?}", result);

    let values = vec![b, time];

    println!("{:?}", mul_vals(&values));
    println!("{:?}", mul_vals(&values));

    println!("{:?}", mul_vals_vec(values.clone()));
    println!("{:?}", mul_vals_vec(values));

    let string: String = String::from("Pierwszy");
    let str_slice: &str = "Drugi";

    let pair_1 = DoubleString::from_strs(&string, str_slice);
    pair_1.show();

    let pair_2 = DoubleString::from_strings(&string, &String::from(str_slice));
    pair_2.show();
}
