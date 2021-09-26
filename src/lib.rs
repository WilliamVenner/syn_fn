#![doc = include_str!("../README.md")]

#[macro_export]
#[doc = include_str!("../README.md")]
macro_rules! syn_squash {
	{ $vis:vis $ident:ident => { default! => {$($fn:tt)+}; $($ty:ty => $code:tt);* } } => {
		trait SynSquashIntoTokens {
			fn into_tokens(&self) -> TokenStream;
		}

		trait SynSquash: SynSquashIntoTokens {
			fn parse(input: TokenStream) -> Option<Box<dyn SynSquash>> where Self: 'static + Sized + syn::parse::Parse {
				syn::parse::<Self>(input.into()).ok().map(|function| Box::new(function) as _)
			}
		
			$($fn)+
		}

		$(
			impl SynSquashIntoTokens for $ty {
				fn into_tokens(&self) -> TokenStream {
					let function: &$ty = self;
					quote!(#function).into()
				}
			}
			impl SynSquash for $ty $code
		)*

		fn $ident(input: TokenStream) -> Option<Box<dyn SynSquash>> {
			$(
				if let Some(function) = <$ty as SynSquash>::parse(input.clone()) {
					return Some(function);
				}
			)*
			None
		}
	};
}