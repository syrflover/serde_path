use serde::{ser, Serialize};

use crate::{error::Error, Result};

struct Serializer {
    map_key: Option<String>,

    output: String,
}

pub fn to_string<T>(path: impl Into<String>, v: &T) -> Result<String>
where
    T: Serialize,
{
    // /users/:user_id Uuid

    let mut serializer = Serializer {
        map_key: None,
        output: path.into(),
    };

    v.serialize(&mut serializer)?;

    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = String;

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<String> {
        let v = if v { "true" } else { "false" };
        Ok(v.to_string())
    }

    // JSON does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same. Other formats, especially compact binary
    // formats, may need independent logic for the different sizes.
    fn serialize_i8(self, v: i8) -> Result<String> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<String> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<String> {
        self.serialize_i64(i64::from(v))
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<String> {
        let v = v.to_string();
        Ok(v)
    }

    fn serialize_u8(self, v: u8) -> Result<String> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<String> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<String> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<String> {
        let v = v.to_string();
        Ok(v)
    }

    fn serialize_f32(self, v: f32) -> Result<String> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<String> {
        let v = v.to_string();
        Ok(v)
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<String> {
        self.serialize_str(&v.to_string())
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, v: &str) -> Result<String> {
        // self.output += v;
        // Ok(v.replace(' ', "-"))
        Ok(v.to_string())
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, _v: &[u8]) -> Result<String> {
        /* use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end() */
        unimplemented!()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<String> {
        self.serialize_unit()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<String> {
        // self.output += "";
        // Ok(String::new())
        unimplemented!()
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        // self.serialize_unit()
        unimplemented!()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self)
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        // self.serialize_seq(Some(len))
        unimplemented!()
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        /* self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":["; */
        // Ok(self)
        unimplemented!()
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        /* self.output += "{";
        Ok(self) */
        // unimplemented!()
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        /* self.output += "{";
        variant.serialize(&mut *self)?;
        self.output += ":{";
        Ok(self) */
        unimplemented!()
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = String;
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(&mut **self)?;

        self.output += &v;
        self.output += "-";

        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<String> {
        if self.output.ends_with('-') {
            self.output.pop();
        }

        Ok(String::new())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let v = value.serialize(&mut **self)?;

        self.output += &v;
        self.output += "-";

        Ok(())
    }

    fn end(self) -> Result<String> {
        if self.output.ends_with('-') {
            self.output.pop();
        }

        Ok(String::new())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        /*
        if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self) */
        unimplemented!()
    }

    fn end(self) -> Result<String> {
        /* self.output += "]";
        Ok(()) */
        unimplemented!()
    }
}

// Tuple variants are a little different. Refer back to the
// `serialize_tuple_variant` method above:
//
//    self.output += "{";
//    variant.serialize(&mut *self)?;
//    self.output += ":[";
//
// So the `end` method in this impl is responsible for closing both the `]` and
// the `}`.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        /* if !self.output.ends_with('[') {
            self.output += ",";
        }
        value.serialize(&mut **self) */
        unimplemented!()
    }

    fn end(self) -> Result<String> {
        /* self.output += "]}";
        Ok(()) */
        unimplemented!()
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // key.serialize(&mut **self)
        // unimplemented!()

        let key = key.serialize(&mut **self)?;

        self.map_key.replace(key);

        Ok(())
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // value.serialize(&mut **self)
        // unimplemented!()

        if let Some(key) = self.map_key.take() {
            let val = value.serialize(&mut **self)?;

            self.output = self.output.replacen(&format!(":{key}"), &val, 1);
        }

        Ok(())
    }

    fn end(self) -> Result<String> {
        // self.output += "}";
        // Ok(())
        // unimplemented!()
        Ok(String::new())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        /*
        if !self.output.ends_with('{') {
            self.output += ",";
        } */
        // key.serialize(&mut **self)?;
        // self.output += ":";
        let val = value.serialize(&mut **self)?;

        self.output = self.output.replacen(&format!(":{key}"), &val, 1);

        Ok(())
    }

    fn end(self) -> Result<String> {
        // self.output += "&";
        Ok(String::new())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = String;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        /* key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;

        Ok(()) */

        unimplemented!()
    }

    fn end(self) -> Result<String> {
        // Ok(self.output)
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use either::Either;
    use serde::Serialize;
    use uuid::Uuid;

    use crate::to_string;

    #[test]
    fn test_str() {
        #[derive(Serialize)]
        struct Test {
            str: String,
        }

        let test = Test {
            str: "test whitespace".to_string(),
        };
        let expected = "/test whitespace/hello";
        assert_eq!(to_string("/:str/hello", &test).unwrap(), expected);
    }

    #[test]
    fn test_hypen() {
        #[derive(Serialize)]
        struct Test {
            str: String,
            x: String,
        }

        let test = Test {
            str: "test whitespace".to_string(),
            x: "world".to_string(),
        };
        let expected = "/test whitespace-world/hello";
        assert_eq!(to_string("/:str-:x/hello", &test).unwrap(), expected);
    }

    #[test]
    fn test_int() {
        #[derive(Serialize)]
        struct Test {
            int: u32,
        }

        let test = Test { int: 1 };
        let expected = "/1/hello";
        assert_eq!(to_string("/:int/hello", &test).unwrap(), expected);
    }

    #[test]
    fn test_tuple() {
        #[derive(Serialize)]
        struct Test {
            tuple: (usize, String),
        }

        let test = Test {
            tuple: (35523, "test whitespace".to_string()),
        };
        let expected = "/35523-test whitespace";
        assert_eq!(to_string("/:tuple", &test).unwrap(), expected);
    }

    #[test]
    fn test_unit_variant() {
        #[derive(Serialize)]
        enum E {
            A,
        }

        #[derive(Serialize)]
        struct Test {
            unit_variant: E,
        }

        let test = Test { unit_variant: E::A };
        let expected = "/A";
        assert_eq!(to_string("/:unit_variant", &test).unwrap(), expected);
    }

    #[test]
    fn test_seq() {
        #[derive(Serialize)]
        struct Test {
            seq: Vec<usize>,
        }

        let test = Test {
            seq: vec![1, 2, 6, 3, 7, 2, 5, 11],
        };
        let expected = "/1-2-6-3-7-2-5-11";
        assert_eq!(to_string("/:seq", &test).unwrap(), expected);
    }

    #[test]
    fn test_uuid() {
        #[derive(Serialize)]
        struct Test {
            user_id: Uuid,
        }

        let test = Test {
            user_id: Uuid::nil(),
        };
        let expected = "/users/00000000-0000-0000-0000-000000000000";
        assert_eq!(to_string("/users/:user_id", &test).unwrap(), expected);
    }

    #[test]
    fn test_either() {
        #[derive(Serialize)]
        struct Test {
            user_id: Either<Uuid, String>,
        }

        let test = Test {
            user_id: Either::Left(Uuid::nil()),
        };
        let expected = "/users/00000000-0000-0000-0000-000000000000";
        assert_eq!(to_string("/users/:user_id", &test).unwrap(), expected);
    }

    #[test]
    fn test_nested_field() {
        #[derive(Serialize)]
        struct Nested {
            s: usize,
        }

        #[derive(Serialize)]
        struct Test {
            user_id: Uuid,
            // #[serde(flatten)]
            // path에는 1차원만 가능하기 때문에
            // flatten을 명시하지 않아도 flatten을 명시한 것처럼 작동함
            nested: Nested,
        }

        let test = Test {
            user_id: Uuid::nil(),
            nested: Nested { s: 125454 },
        };
        let expected = "/users/00000000-0000-0000-0000-000000000000/125454";
        assert_eq!(to_string("/users/:user_id/:s", &test).unwrap(), expected);
    }
}
