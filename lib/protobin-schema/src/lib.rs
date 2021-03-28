use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse2, parse_quote, spanned::Spanned, Data, DeriveInput, Error, Fields, GenericParam, Generics,
};

pub use proc_macro2;

pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = match parse2::<DeriveInput>(input) {
        Ok(data) => data,
        Err(err) => return err.to_compile_error(),
    };

    let name = input.ident;
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let serialize_body = match serialize_impl(&input.data) {
        Ok(data) => data,
        Err(err) => return Error::new(Span::call_site(), err).to_compile_error(),
    };

    quote! {
        impl #impl_generics protobin::Serialize for #name #ty_generics #where_clause {
            fn wire_type(&self) -> protobin::WireType {
                protobin::WireType::VarLen
            }

            fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
                #serialize_body
                Ok(())
            }
        }

        impl #impl_generics protobin::Deserialize for #name #ty_generics #where_clause {
            fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
                todo!()
            }
        }
    }
}

// Add a bound `T: Serialize + Deserialize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param
                .bounds
                .push(parse_quote!(protobin::Serialize + protobin::Deserialize));
        }
    }
    generics
}

fn serialize_impl(data: &Data) -> Result<TokenStream, &'static str> {
    match data {
        Data::Struct(struct_data) => match &struct_data.fields {
            Fields::Named(fields) => {
                let len = fields.named.len() as u32;

                let recurse = fields.named.iter().enumerate().map(|(idx, field)| {
                    let idx = (idx + 1) as u32;
                    let name = &field.ident;

                    quote_spanned! { field.span() =>
                        serializer.write_field(#idx, &self.#name)?;
                    }
                });

                Ok(quote! {
                    serializer.write_u32(#len)?;
                    #(#recurse)*
                })
            }
            Fields::Unnamed(_) | Fields::Unit => Err("expected struct with named field"),
        },
        Data::Enum(_) => {
            todo!()
        }
        Data::Union(_) => Err("this trait cannot be derived for unions"),
    }
}
