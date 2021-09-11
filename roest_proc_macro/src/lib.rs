use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Ft" => "Err",
        "Goed" => "Ok",
        "Keten" => "String",
        "ktng" => "str",
        "Woordenboek" => "HashMap",
        "Standaard" => "Default",
        "Fout" => "Error",
        "Mogelijkheid" => "Option",
        "Enige" => "Some",
        "Geen" => "None",
        "Resultaat" => "Result",
        "Zelf" => "Self",
        "schrijfrgl" => "println",
        "ontsnap" => "break",
        "gelijktijdige" => "async",
        "wacht_af" => "await",
        "lus" => "loop",
        "verplaats" => "move",
        "krat" => "crate",
        "zoals" => "as",
        "onveranderlijk" => "const",
        "gelijkend" => "match",
        "gevaarlijk" => "unsafe",
        "binnen" => "in",
        "van" => "from",
        "dynamisch" => "dyn",
        "pak_uit" => "unwrap",
        "standaard" => "default",
        "iu" => "io",
        "extern" => "extern",
        "onwaar" => "false",
        "functie" => "fn",
        "bovenstaand" => "super",
        "uitwerking" => "impl",
        "voeg_in" => "insert",
        "karaktereigenschap" => "trait",
        "verkrijg" => "get",
        "module" => "mod",
        "veranderlijk" => "mut",
        "nieuw" => "new",
        "gegeven" => "where",
        "voor" => "for",
        "verkrijg_of_voeg_toe_met" => "get_or_insert_with",
        "hoofd" => "main",
        "openbaar" => "pub",
        "geef_terug" => "return",
        "als" => "if",
        "anders" => "else",
        "zelf" => "self",
        "laat" => "let",
        "vast" => "static",
        "structuur" => "struct",
        "verwacht" => "expect",
        "zolang" => "while",
        "gebruik" => "use",
        "waar" => "true",
        "opsomming" => "enum",

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
