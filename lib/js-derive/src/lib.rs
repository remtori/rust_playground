use quote::quote;
use synstructure::{decl_derive, BindStyle, Structure};

decl_derive!([GcTrace, attributes(unsafe_ignore_trace)] => derive_trace);

fn derive_trace(mut s: Structure<'_>) -> proc_macro2::TokenStream {
    s.filter(|bi| {
        !bi.ast()
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("unsafe_ignore_trace"))
    });
    s.bind_with(|_bi| BindStyle::RefMut);
    let trace_body = s.each(|bi| quote!(mark(#bi,tracer)));

    let trace_impl = s.gen_impl(quote! {

        gen unsafe impl Trace for @Self {
        #[inline] fn trace(&mut self,tracer: &mut Tracer) {
            #[allow(dead_code)]
            #[inline]
            fn mark<T: Trace + ?Sized>(it: &mut T, tracer: &mut Tracer) {
              it.trace(tracer);
            }
            match &mut*self { #trace_body }
        }
    }

    });
    quote! {
        #trace_impl
    }
}
