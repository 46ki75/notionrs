use serde::{Deserialize, Serialize};

/// The types of programming languages available can be found in the code of
/// [the repository](https://github.com/makenotion/notion-sdk-js/blob/7950edc034d3007b0612b80d3f424baef89746d9/src/api-endpoints.ts#L5331),
/// which is more up-to-date than the Notion API documentation.
///
/// Please look for the union type of the `type LanguageRequest` string in the codebase of `notion-sdk-js`.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    #[serde(rename = "abap")]
    Abap,
    #[serde(rename = "agda")]
    Agda,
    #[serde(rename = "arduino")]
    Arduino,
    #[serde(rename = "assembly")]
    Assembly,
    #[serde(rename = "bash")]
    Bash,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "bnf")]
    Bnf,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "c#")]
    CSharp,
    #[serde(rename = "c++")]
    CPlusPlus,
    #[serde(rename = "clojure")]
    Clojure,
    #[serde(rename = "coffeescript")]
    Coffeescript,
    #[serde(rename = "coq")]
    Coq,
    #[serde(rename = "css")]
    Css,
    #[serde(rename = "dart")]
    Dart,
    #[serde(rename = "dhall")]
    Dhall,
    #[serde(rename = "diff")]
    Diff,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "ebnf")]
    Ebnf,
    #[serde(rename = "elixir")]
    Elixir,
    #[serde(rename = "elm")]
    Elm,
    #[serde(rename = "erlang")]
    Erlang,
    #[serde(rename = "f#")]
    FSharp,
    #[serde(rename = "flow")]
    Flow,
    #[serde(rename = "fortran")]
    Fortran,
    #[serde(rename = "gherkin")]
    Gherkin,
    #[serde(rename = "glsl")]
    Glsl,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "graphql")]
    Graphql,
    #[serde(rename = "groovy")]
    Groovy,
    #[serde(rename = "haskell")]
    Haskell,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "idris")]
    Idris,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "julia")]
    Julia,
    #[serde(rename = "kotlin")]
    Kotlin,
    #[serde(rename = "latex")]
    Latex,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "lisp")]
    Lisp,
    #[serde(rename = "livescript")]
    Livescript,
    #[serde(rename = "llvm ir")]
    LlvmIr,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "makefile")]
    Makefile,
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "markup")]
    Markup,
    #[serde(rename = "matlab")]
    Matlab,
    #[serde(rename = "mathematica")]
    Mathematica,
    #[serde(rename = "mermaid")]
    Mermaid,
    #[serde(rename = "nix")]
    Nix,
    #[serde(rename = "notion formula")]
    NotionFormula,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    #[serde(rename = "ocaml")]
    Ocaml,
    #[serde(rename = "pascal")]
    Pascal,
    #[serde(rename = "perl")]
    Perl,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "plain text")]
    #[default]
    PlainText,
    #[serde(rename = "powershell")]
    Powershell,
    #[serde(rename = "prolog")]
    Prolog,
    #[serde(rename = "protobuf")]
    Protobuf,
    #[serde(rename = "purescript")]
    Purescript,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "racket")]
    Racket,
    #[serde(rename = "reason")]
    Reason,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "sass")]
    Sass,
    #[serde(rename = "scala")]
    Scala,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "scss")]
    Scss,
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "solidity")]
    Solidity,
    #[serde(rename = "sql")]
    Sql,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "toml")]
    Toml,
    #[serde(rename = "typescript")]
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    #[serde(rename = "verilog")]
    Verilog,
    #[serde(rename = "vhdl")]
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    #[serde(rename = "webassembly")]
    Webassembly,
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaCCPlusPlusCSharp,
}

impl std::str::FromStr for Language {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s)
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_plain::to_string(self).unwrap())
    }
}
