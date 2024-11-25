struct ColorRegularStruct {
    red: i32,
    grn: i32,
    blue: i32,
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)]
struct UnitStruct;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct{
            grn: 255,
            red: 0,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.grn, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
         let green =ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
