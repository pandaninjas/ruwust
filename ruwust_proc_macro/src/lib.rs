use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Ewo" => "Err",
        "Goed" => "Ok",
        "Stwing" => "String",
        "stw" => "str",
        "HawshMap" => "HashMap",
        "Defauwt" => "Default",
        "Ewwow" => "Error",
        "Option" => "Option",
        "Some" => "Some",
        "None" => "None",
        "Result" => "Result",
        "Self" => "Self",
        "println" => "println",
        "break" => "break",
        "async" => "async",
        "await" => "await",
        "loop" => "loop",
        "move" => "move",
        "crate" => "crate",
        "as" => "as",
        "const" => "const",
        "match" => "match",
        "unsafe" => "unsafe",
        "in" => "in",
        "from" => "from",
        "dyn" => "dyn",
        "unwrap" => "unwrap",
        "default" => "default",
        "io" => "io",
        "extern" => "extern",
        "false" => "false",
        "fn" => "fn",
        "super" => "super",
        "impl" => "impl",
        "insert" => "insert",
        "trait" => "trait",
        "get" => "get",
        "mod" => "mod",
        "mut" => "mut",
        "new" => "new",
        "where" => "where",
        "for" => "for",
        "get_or_insert_with" => "get_or_insert_with",
        "main" => "main",
        "pub" => "pub",
        "return" => "return",
        "if" => "if",
        "else" => "else",
        "self" => "self",
        "let" => "let",
        "static" => "static",
        "struct" => "struct",
        "expect" => "expect",
        "while" => "while",
        "use" => "use",
        "true" => "true",
        "enum" => "enum",
        "into" => "into",
        "ref" => "ref",
        "as_ref" => "as_ref",
        "unweachabwe_code" => "unreachable_code",
        "awwow" => "allow",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn roest(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
