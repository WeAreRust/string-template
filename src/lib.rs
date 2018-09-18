// TODO: Do better.
#[allow(unused_imports)]
use pest_derive::*;

// Forces the grammar to be rebuilt when compiling in debug mode. For more info see
// https://docs.rs/pest_derive/1.0.8/pest_derive/#pest-files
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct TemplateParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    fn parse(rule: Rule, content: &str) -> Vec<(Rule, &str)> {
        let pairs = TemplateParser::parse(rule, content).unwrap();
        pairs.map(|pair| (pair.as_rule(), pair.as_str())).collect()
    }

    #[test]
    fn template_decl_no_parameters() {
        let parsed = parse(
            Rule::template,
            "tmpl_ident ::= <<\n\
             tmpl content\n\
             >>",
        );
        assert_eq!(
            parsed,
            [
                (Rule::ident, "tmpl_ident"),
                (Rule::content, "tmpl content\n")
            ]
        );
    }
}
