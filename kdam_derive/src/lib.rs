use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Meta, Path};

/// Derive [BarExt](https://docs.rs/kdam/latest/kdam/trait.BarExt.html) trait for a struct.
///
/// # Example
///
/// ```no_test
/// use kdam::{Bar, BarExt};
///
/// #[derive(BarExt)]
/// struct CustomBar {
///     #[bar]
///     pb: Bar,
/// }
///
/// impl CustomBar {
///     fn render(&mut self) -> String {
///         format!(
///             "Progress: {}/{}",
///             self.pb.fmt_counter(),
///             self.pb.fmt_total(),
///         )
///     }
/// }
/// ```
#[proc_macro_derive(BarExt, attributes(bar))]
pub fn bar_ext(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut bar_field = None;

    if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named, .. }),
        ..
    }) = &input.data
    {
        for field in named {
            for attr in &field.attrs {
                if let Meta::Path(Path { segments, .. }) = &attr.meta {
                    for segment in segments {
                        if &segment.ident.to_string() == "bar" {
                            bar_field = Some(field.ident.clone());
                        }
                    }
                }
            }
        }
    } else {
        unimplemented!("BarExt derive macro is only derivable on structs.")
    }

    if bar_field.is_none() {
        panic!("One struct field needs to use #[bar] attribute.")
    }

    let crate_name = if std::env::var("CARGO_CRATE_NAME")
        .expect("CARGO_CRATE_NAME env variable not set by cargo.")
        == "kdam"
    {
        "crate"
    } else {
        "kdam"
    };
    let crate_name = format_ident!("{}", crate_name);

    let bar_field = bar_field
        .flatten()
        .expect("#[bar] attribute struct field has not a valid identifier.");
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #crate_name::BarExt for #name #ty_generics #where_clause {
            fn clear(&mut self) {
                self.#bar_field.clear();
            }

            fn input<T: Into<String>>(&mut self, text: T) -> Result<String, std::io::Error> {
                self.clear();
                self.#bar_field.writer.print_str(&text.into());

                let mut input_string = String::new();
                std::io::stdin().read_line(&mut input_string)?;

                if self.#bar_field.leave {
                    self.refresh();
                }

                Ok(input_string)
            }

            fn refresh(&mut self) {
                if !self.#bar_field.force_refresh {
                    self.#bar_field.force_refresh = true;
                    self.update(0);
                    self.#bar_field.force_refresh = false;
                } else {
                    self.update(0);
                }
            }

            fn render(&mut self) -> String {
                Self::render(self)
            }

            fn reset(&mut self, total: Option<usize>) {
                self.#bar_field.reset(total);
            }

            fn update(&mut self, n: usize) -> bool {
                if self.#bar_field.trigger(n) {
                    let text = self.render();
                    let length = #crate_name::term::Colorizer::len_ansi(text.as_str()) as i16;

                    if length != self.#bar_field.get_bar_length() {
                        self.#bar_field.clear();
                    }

                    self.#bar_field.set_bar_length(length);
                    self.#bar_field.write_at(text);
                    return true;
                }

                false
            }

            fn update_to(&mut self, update_to_n: usize) -> bool {
                self.#bar_field.set_counter(update_to_n);
                self.update(0)
            }

            fn write<T: Into<String>>(&mut self, text: T) {
                self.#bar_field.clear();
                self.#bar_field.writer.print(format_args!("\r{}\n", text.into()));

                if self.#bar_field.leave {
                    self.refresh();
                }
            }

            fn write_to<T: std::io::Write>(&mut self, writer: &mut T, n: Option<usize>) -> bool {
                let text;

                if let Some(n) = &n {
                    if self.#bar_field.trigger(*n) {
                        text = #crate_name::term::Colorizer::trim_ansi(self.render().as_str());
                    } else {
                        return false;
                    }
                } else {
                    text = #crate_name::term::Colorizer::trim_ansi(self.render().as_str());
                }

                self.#bar_field
                    .set_bar_length(#crate_name::term::Colorizer::len_ansi(text.as_str()) as i16);
                #crate_name::lock::acquire();
                writer.write_fmt(format_args!("{}\n", text)).unwrap();
                writer.flush().unwrap();
                #crate_name::lock::release();
                true
            }
        }
    };

    TokenStream::from(expanded)
}
