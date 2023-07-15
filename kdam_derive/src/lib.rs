use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Meta, Path};

/// Derive [BarExt](https://docs.rs/kdam/latest/kdam/trait.BarExt.html) trait for a struct.
///
/// # Example
///
/// ```no_test
/// use kdam::{tqdm, Bar, BarExt};
/// use std::{io::Result, num::NonZeroU16};
/// 
/// #[derive(BarExt)]
/// struct CustomBar {
///     #[bar]
///     pb: Bar,
/// }
/// 
/// impl CustomBar {
///     /// Render progress bar text.
///     fn render(&mut self) -> String {
///         let fmt_percentage = self.pb.fmt_percentage(0);
///         let padding = 1 + fmt_percentage.chars().count() as u16 + self.pb.animation.spaces() as u16;
/// 
///         let ncols = self.pb.ncols_for_animation(padding);
/// 
///         if ncols == 0 {
///             self.pb.bar_length = padding - 1;
///             fmt_percentage
///         } else {
///             self.pb.bar_length = padding + ncols;
///             self.pb.animation.fmt_render(
///                 NonZeroU16::new(ncols).unwrap(),
///                 self.pb.percentage(),
///                 &None,
///             ) + " "
///                 + &fmt_percentage
///         }
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
            fn clear(&mut self) -> ::std::io::Result<()> {
                self.#bar_field.clear()
            }

            fn input<T: Into<String>>(&mut self, text: T) -> ::std::io::Result<String> {
                self.clear()?;
                self.#bar_field.writer.print(text.into().as_bytes())?;

                let mut buf = String::new();
                ::std::io::stdin().read_line(&mut buf)?;

                if self.#bar_field.leave {
                    self.refresh()?;
                }

                Ok(buf)
            }

            fn refresh(&mut self) -> ::std::io::Result<()> {
                if !self.#bar_field.force_refresh {
                    self.#bar_field.force_refresh = true;
                    self.update(0)?;
                    self.#bar_field.force_refresh = false;
                } else {
                    self.update(0)?;
                }

                Ok(())
            }

            fn render(&mut self) -> String {
                Self::render(self)
            }

            fn reset(&mut self, total: Option<usize>) {
                self.#bar_field.reset(total);
            }

            fn update(&mut self, n: usize) -> ::std::io::Result<bool> {
                self.#bar_field.counter += n;

                if self.#bar_field.should_refresh() {
                    let text = self.render();
                    let length = #crate_name::term::Colorizer::len_ansi(text.as_str()) as u16;

                    if length != self.#bar_field.bar_length {
                        self.#bar_field.clear()?;
                    }

                    self.#bar_field.bar_length = length;
                    self.#bar_field.writer.print_at(self.pb.position, text.as_bytes())?;
                    return Ok(true);
                }

                Ok(false)
            }

            fn update_to(&mut self, update_to_n: usize) -> ::std::io::Result<bool> {
                self.#bar_field.counter = update_to_n;
                self.update(0)
            }

            fn write<T: Into<String>>(&mut self, text: T) -> ::std::io::Result<()> {
                self.#bar_field.clear()?;
                self.#bar_field.writer.print(format!("\r{}\n", text.into()).as_bytes())?;

                if self.#bar_field.leave {
                    self.refresh()?;
                }

                Ok(())
            }

            fn write_to<T: ::std::io::Write>(&mut self, writer: &mut T, n: Option<usize>) -> ::std::io::Result<bool> {
                let text;

                if let Some(n) = n {
                    self.#bar_field.counter += n;

                    if self.#bar_field.should_refresh() {
                        text = #crate_name::term::Colorizer::trim_ansi(self.render().as_str());
                    } else {
                        return Ok(false);
                    }
                } else {
                    text = #crate_name::term::Colorizer::trim_ansi(self.render().as_str());
                }

                self.#bar_field.bar_length = #crate_name::term::Colorizer::len_ansi(text.as_str()) as u16;
                #crate_name::lock::acquire();
                writer.write_all((text + "\n").as_bytes())?;
                writer.flush()?;
                #crate_name::lock::release();
                Ok(true)
            }
        }
    };

    TokenStream::from(expanded)
}
