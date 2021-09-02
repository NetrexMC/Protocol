use proc_macro::TokenStream;
use quote::{quote};
use syn::{Data, DeriveInput, Fields};

pub fn derive_encoder(input: DeriveInput) -> TokenStream {
     let name = input.ident;
     let writes: TokenStream = match &input.data {
          Data::Struct(data) => {
               parse_fields(&data.fields)
          },
          _ => panic!("Can not parse this field as it is invalid!")
     };

     TokenStream::from(quote!(
          impl StreamEncoder for #name {
               fn into_stream(&self) -> BinaryStream {
                    let streams = Vec::new(#writes);
                    let mut main_stream = BinaryStream::new();
                    for stream in streams {
                         main_stream.write_slice(stream.get_buffer());
                    }
               }
          }
     ))
}

fn parse_fields(fields: &Fields) -> TokenStream {
     let mut stream = Vec::new();

     match fields {
          Fields::Named(fields) => {
               for field in fields.named {
                    let id = field.ident.as_ref();
                    stream.push(quote!(self.#id.into_stream()));
               }
          },
          Fields::Unnamed(fields) => panic!("Can not parse unnamed fields.")
     }
     TokenStream::from(quote!(#(#stream)*))
}