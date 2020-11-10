#![feature(const_ptr_offset)]

mod array;
mod number;
mod string;
mod value;
pub use array::IArray;
pub use number::INumber;
pub use string::IString;
pub use value::IValue;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
