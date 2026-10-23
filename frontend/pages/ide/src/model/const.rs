/// The language the scratchpad opens with when the query parameter names no
/// language the editor knows.
pub(crate) const DEFAULT_LANGUAGE: &str = "cpp";

/// The query parameter that names the language of the scratchpad.
pub(crate) const QUERY_LANGUAGE: &str = "language";

/// The prefix of the key the code of a language is kept under.
pub(crate) const STORAGE_PREFIX: &str = "[code]";

/// The languages the scratchpad holds, with the name they are shown under.
pub(crate) const LANGUAGES: &[(&str, &str)] = &[
    ("c", "C"),
    ("cpp", "C++"),
    ("php", "PHP"),
    ("java", "Java"),
    ("javascript", "JavaScript"),
    ("typescript", "TypeScript"),
    ("ruby", "Ruby"),
    ("rust", "Rust"),
    ("python", "Python3"),
    ("go", "Go"),
    ("csharp", "C#"),
];

/// The aliases the query parameter of the language accepts, with the
/// identifier of the language they name.
pub(crate) const ALIASES: &[(&str, &str)] = &[
    ("c", "c"),
    ("c++", "cpp"),
    ("cpp", "cpp"),
    ("rs", "rust"),
    ("rust", "rust"),
    ("php", "php"),
    ("inc", "php"),
    ("java", "java"),
    ("js", "javascript"),
    ("javascript", "javascript"),
    ("node", "javascript"),
    ("nodejs", "javascript"),
    ("typescript", "typescript"),
    ("ts", "typescript"),
    ("golang", "go"),
    ("go", "go"),
    ("py", "python"),
    ("python", "python"),
    ("python2", "python"),
    ("python3", "python"),
    ("rusthon", "python"),
    ("ruby", "ruby"),
    ("jruby", "ruby"),
    ("macruby", "ruby"),
    ("rake", "ruby"),
    ("rb", "ruby"),
    ("rbx", "ruby"),
    ("csharp", "csharp"),
    ("c#", "csharp"),
];
