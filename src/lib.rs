use proc_macro::TokenStream;
use quote::quote;
use syn::{Fields, Item, ItemEnum, ItemStruct};

mod cosmwasm_schema_gen;
#[proc_macro_attribute] // impl ToAscObj<Type> for Type
pub fn cosmwasm_schema_gen(args: TokenStream, input: TokenStream) -> TokenStream {
    cosmwasm_schema_gen::cosmwasm_schema_gen(args, input)
}









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
