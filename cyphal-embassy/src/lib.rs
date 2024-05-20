#![doc = include_str!("../README.md")]
// #![forbid(missing_docs)]

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

/// Macro from Embassy Bxcan interfaces
#[proc_macro_derive(Bxcan)]
pub fn embassy_bxcan_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics cyphal_can::Can<8> for #name #ty_generics #where_clause{
            type Frame = CyphalFrame;

            async fn transmit(&mut self, frame: &Self::Frame) -> cyphal_can::CanResult<()> {
                Err(cyphal_can::CanError::Other)
            }

            async fn receive(&mut self) -> cyphal_can::CanResult<Self::Frame> {
                Err(cyphal_can::CanError::Other)
            }
        }

        pub struct CyphalFrame {
            can_id: CanId,
            frame: BxcanFrame,
        }

        impl CyphalFrame {
            pub(crate) fn inner_frame(&self) -> &bxcan::Frame {
                &self.frame
            }
        }

        impl cyphal_can::Frame<8> for CyphalFrame {
            fn new(id: impl Into<cyphal_can::CanId>, data: &[u8]) -> cyphal_can::CanResult<Self> {
                let can_id: cyphal_can::CanId = id.into();
                let extended_id = bxcan::ExtendedId::new(can_id.as_raw()).unwrap();
                match data.len() {
                    n if n <= CLASSIC_PAYLOAD_SIZE => {
                        let mut bytes: [u8; CLASSIC_PAYLOAD_SIZE] = [0; CLASSIC_PAYLOAD_SIZE];
                        bytes[..n].copy_from_slice(data);
                        let frame = bxcan::Frame::new_data(extended_id, bxcan::Data::new(data).unwrap());
                        Ok(CyphalFrame { can_id, frame })
                    }
                    _ => Err(cyphal_can::CanError::Other),
                }
            }

            fn id(&self) -> cyphal_can::CanId {
                self.can_id
            }

            fn dlc(&self) -> usize {
                match self.frame.data() {
                    Some(data) => data.len(),
                    None => 0,
                }
            }

            fn data(&self) -> &[u8] {
                match self.frame.data() {
                    Some(data) => data,
                    None => &[],
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(cyphal_can::Can));
        }
    }
    generics
}
