// TODO: Do better than `*` import.
#[allow(unused_imports)]
use pest_derive::*;

// Forces the grammar to be rebuilt when compiling in debug mode. For more info see
// https://docs.rs/pest_derive/1.0.8/pest_derive/#pest-files
#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "group/grammar.pest"]
pub struct GroupParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    fn parse(rule: Rule, content: &str) -> Vec<(Rule, &str)> {
        let pairs = GroupParser::parse(rule, content).unwrap();
        pairs.map(|pair| (pair.as_rule(), pair.as_str())).collect()
    }

    #[test]
    fn decl_no_parameters() {
        let parsed = parse(Rule::decl, "T ::= <<");
        assert_eq!(parsed, [(Rule::ident, "T")]);
    }

    #[test]
    fn decl_empty_parameters() {
        let parsed = parse(Rule::decl, "T() ::= <<");
        assert_eq!(parsed, [(Rule::ident, "T")]);
    }

    #[test]
    fn decl_one_parameter() {
        let parsed = parse(Rule::decl, "T(p1) ::= <<");
        assert_eq!(parsed, [(Rule::ident, "T"), (Rule::param, "p1"),]);
    }

    #[test]
    fn decl_multiple_parameters() {
        let parsed = parse(Rule::decl, "T(p1, p2) ::= <<");
        assert_eq!(
            parsed,
            [(Rule::ident, "T"), (Rule::param, "p1"), (Rule::param, "p2")]
        );
    }

    #[test]
    fn basic_template() {
        let parsed = parse(Rule::group, "T ::= <<content>>");
        assert_eq!(parsed, [(Rule::ident, "T"), (Rule::template, "content")]);
    }

    #[test]
    fn empty_template() {
        let parsed = parse(Rule::group, "T ::= <<>>");
        assert_eq!(parsed, [(Rule::ident, "T")]);
    }

    #[test]
    fn escaped_template() {
        let parsed = parse(Rule::group, "T ::= <<\\<<content\\>>>>");
        assert_eq!(
            parsed,
            [(Rule::ident, "T"), (Rule::template, "\\<<content\\>>")]
        );
    }

    #[test]
    fn group_file() {
        let parsed = parse(Rule::file, "A ::= <<c_A>>\nB ::= <<c_B>>");
        assert_eq!(
            parsed,
            [
                (Rule::ident, "A"),
                (Rule::template, "c_A"),
                (Rule::ident, "B"),
                (Rule::template, "c_B")
            ]
        );
    }
}
