use proc_macro2::TokenStream;
use syn::parse::ParseStream;
use syn::parse::Parse;
use syn::parse::Result;
use syn::buffer::Cursor;
use syn::Block;
use quote::ToTokens;
use proc_macro2::TokenTree;
use proc_macro2::Delimiter;

fn extract_js(cursor: Cursor) -> Result<(Js, Cursor)> {
  let mut script = String::new();
  let mut blocks = Vec::new();

  let mut rest = cursor;
  let mut block_counter = 0;
  let mut next_is_block = false;

  while let Some((tt, next)) = rest.token_tree() {
    if next_is_block {
      let ts = tt.into_token_stream();
      blocks.push(match ::syn::parse2::<Block>(ts) {
        Ok(block) => block,
        Err(err) => return Err(err),
      });

      script += "$";
      script += &block_counter.to_string();

      block_counter += 1;
      next_is_block = false;

      rest = next;
      continue;
    }

    match tt {
      TokenTree::Punct(ref punct) if punct.as_char() == '@' && next.group(Delimiter::Brace).is_some() => {
        next_is_block = true;
      },
      TokenTree::Group(ref group) => {

        let (start, end) = match group.delimiter() {
          Delimiter::Brace => ("{", "}"),
          Delimiter::Bracket => ("[", "]"),
          Delimiter::Parenthesis => ("(", ")"),
          Delimiter::None => ("", ""),
        };

        let (group, _, next) = rest.group(group.delimiter()).unwrap();
        rest = next;

        let mut js = extract_js(group)?.0;

        script += start;
        script += &js.script;
        script += end;
        blocks.append(&mut js.blocks);

        continue;
      },
      _ => {
        script += &tt.to_string();
        script += " ";
      }
    }

    rest = next;
  }

  Ok((Js {
    script,
    blocks,
  }, rest))
}

#[derive(Debug)]
pub struct Js {
  script: String,
  blocks: Vec<Block>,
}

impl Parse for Js {
  fn parse(input: ParseStream) -> Result<Self> {
    input.step(|cursor| {
      extract_js(*cursor)
    })
  }
}

pub fn transform_js(input: Js) -> TokenStream {
  let Js { script, blocks } = input;

  let script_str = script.to_string();

  quote!(__js_raw_asm!(#script_str, #(#blocks)*))
}