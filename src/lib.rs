extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
/*
#[proc_macro_attribute]
pub fn yaserde_caldav(attr: TokenStream, item: TokenStream) -> TokenStream {
    let gen = quote! {
        #[yaserde(
            ,attr
            namespace = "d: DAV:",
            namespace = "s: http://sabredav.org/ns",
            namespace = "cal: urn:ietf:params:xml:ns:caldav",
            namespace = "cs: http://calendarserver.org/ns/",
            namespace = "oc: http://owncloud.org/ns",
            namespace = "nc: http://nextcloud.org/ns"
        )]
       ,item
    };
    gen.into()
}
*/
