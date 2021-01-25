// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            name: String::from("green"),
            hex:  String::from("#00FF00"),
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // This works
        let green = ColorTupleStruct( 
             String::from("green"),
             String::from("#00FF00") 
        );
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");

        // This also works, but it's just a tuple (not a tuple struct)
        let green = (
            String::from("green"),
            String::from("#00FF00") ,
        );
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");

        // This does not work (because these are raw strs, not full Strings, so they mistmatch what is in the ColorTupleStruct)
        // let green = ColorTupleStruct("green", "#00FF00");
        // assert_eq!(green.0, "green");
        // assert_eq!(green.1, "#00FF00");

        // But THIS does work  Weird, right?  Well, it's just a tuple, not a tuple struct, so there's no mismatch on types
        let green = ("green", "#00FF00");
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");

        // This also works
        let color = String::from("green");
        let code = String::from("#00FF00");
        let green = ColorTupleStruct(color,code);
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");

        // And so does this... once again, it's just a tuple, not a tuple struct
        let color = String::from("green");
        let code = String::from("#00FF00");
        let green = (color,code);
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
