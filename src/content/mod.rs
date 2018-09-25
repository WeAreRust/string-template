// TODO: Do better than `*` import.
#[allow(unused_imports)]
use pest_derive::*;

// Forces the grammar to be rebuilt when compiling in debug mode. For more info see
// https://docs.rs/pest_derive/1.0.8/pest_derive/#pest-files
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "content/grammar.pest"]
pub struct ContentParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    fn parse(rule: Rule, content: &str) -> Vec<(Rule, &str)> {
        let pairs = ContentParser::parse(rule, content).unwrap();
        pairs.map(|pair| (pair.as_rule(), pair.as_str())).collect()
    }

    #[test]
    fn raw_content() {
        let parsed = parse(Rule::content, "struct S{\n\titem: String,\n}");
        assert_eq!(parsed, [(Rule::raw, "struct S{\n\titem: String,\n}")]);
    }

    #[test]
    fn directive_content() {
        let parsed = parse(Rule::content, "a <directive> b");
        assert_eq!(
            parsed,
            [
                (Rule::raw, "a "),
                (Rule::directive, "<directive>"),
                (Rule::raw, " b")
            ]
        );
    }

    #[test]
    fn escaped_directive_in_raw() {
        let parsed = parse(Rule::content, "a \\<directive\\> b");
        assert_eq!(parsed, [(Rule::raw, "a \\<directive\\> b"),]);
    }

    #[test]
    fn escaped_directive_in_directive() {
        let parsed = parse(Rule::content, "a <a \\<directive \\> b> b");
        assert_eq!(
            parsed,
            [
                (Rule::raw, "a "),
                (Rule::directive, "<a \\<directive \\> b>"),
                (Rule::raw, " b")
            ]
        );
    }
}
