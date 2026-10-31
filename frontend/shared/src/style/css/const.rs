/// The name of the Monaco theme of the light pair.
pub(crate) const MONACO_THEME_LIGHT: &str = "euv-light";

/// The name of the Monaco theme of the dark pair.
pub(crate) const MONACO_THEME_DARK: &str = "euv-dark";

/// The rules of the light pair of the editor.
///
/// The system is monochrome: no colour, only weight. Every token the editor
/// can paint is listed here, including the bracket token. A token that is
/// missing from the list keeps the colour of the base theme, and the base
/// theme of Monaco is colourful.
pub(crate) const MONACO_LIGHT_RULES: &str = r#"[
    { token: 'comment', foreground: '#8a8a8a', fontStyle: 'italic' },
    { token: 'keyword', foreground: '#000000' },
    { token: 'string', foreground: '#000000' },
    { token: 'number', foreground: '#000000' },
    { token: 'type', foreground: '#000000' },
    { token: 'type.identifier', foreground: '#000000' },
    { token: 'identifier', foreground: '#000000' },
    { token: 'delimiter', foreground: '#000000' },
    { token: 'delimiter.bracket', foreground: '#000000' },
    { token: 'delimiter.array', foreground: '#000000' },
    { token: 'operator', foreground: '#000000' },
    { token: 'variable', foreground: '#000000' },
    { token: 'variable.predefined', foreground: '#000000' },
    { token: 'invalid', foreground: '#000000' },
]"#;

/// The rules of the dark pair of the editor.
///
/// The same tokens, with the ink of the dark theme.
pub(crate) const MONACO_DARK_RULES: &str = r#"[
    { token: 'comment', foreground: '#8a8a8a', fontStyle: 'italic' },
    { token: 'keyword', foreground: '#ffffff' },
    { token: 'string', foreground: '#ffffff' },
    { token: 'number', foreground: '#ffffff' },
    { token: 'type', foreground: '#ffffff' },
    { token: 'type.identifier', foreground: '#ffffff' },
    { token: 'identifier', foreground: '#ffffff' },
    { token: 'delimiter', foreground: '#ffffff' },
    { token: 'delimiter.bracket', foreground: '#ffffff' },
    { token: 'delimiter.array', foreground: '#ffffff' },
    { token: 'operator', foreground: '#ffffff' },
    { token: 'variable', foreground: '#ffffff' },
    { token: 'variable.predefined', foreground: '#ffffff' },
    { token: 'invalid', foreground: '#ffffff' },
]"#;

/// The colours of the light pair of the editor.
pub(crate) const MONACO_LIGHT_COLORS: &str = r#"{
    'editor.background': '#ffffff',
    'editor.foreground': '#000000',
    'editorLineNumber.foreground': '#8a8a8a',
    'editorLineNumber.activeForeground': '#000000',
    'editorCursor.foreground': '#000000',
    'editor.selectionBackground': '#00000033',
    'editor.inactiveSelectionBackground': '#0000001a',
    'editor.lineHighlightBackground': '#0000000a',
    'editorIndentGuide.background1': '#0000001a',
    'editorIndentGuide.activeBackground1': '#00000040',
    'editorWidget.background': '#ffffff',
    'editorWidget.border': '#000000',
    'editorGutter.background': '#ffffff',
    'scrollbarSlider.background': '#00000033',
    'scrollbarSlider.hoverBackground': '#00000055',
    'scrollbarSlider.activeBackground': '#00000077',
    'editorBracketHighlight.foreground1': '#000000',
    'editorBracketHighlight.foreground2': '#000000',
    'editorBracketHighlight.foreground3': '#000000',
    'editorBracketHighlight.foreground4': '#000000',
    'editorBracketHighlight.foreground5': '#000000',
    'editorBracketHighlight.foreground6': '#000000',
}"#;

/// The colours of the dark pair of the editor.
pub(crate) const MONACO_DARK_COLORS: &str = r#"{
    'editor.background': '#000000',
    'editor.foreground': '#ffffff',
    'editorLineNumber.foreground': '#8a8a8a',
    'editorLineNumber.activeForeground': '#ffffff',
    'editorCursor.foreground': '#ffffff',
    'editor.selectionBackground': '#ffffff33',
    'editor.inactiveSelectionBackground': '#ffffff1a',
    'editor.lineHighlightBackground': '#ffffff0a',
    'editorIndentGuide.background1': '#ffffff1a',
    'editorIndentGuide.activeBackground1': '#ffffff40',
    'editorWidget.background': '#000000',
    'editorWidget.border': '#ffffff',
    'editorGutter.background': '#000000',
    'scrollbarSlider.background': '#ffffff33',
    'scrollbarSlider.hoverBackground': '#ffffff55',
    'scrollbarSlider.activeBackground': '#ffffff77',
    'editorBracketHighlight.foreground1': '#ffffff',
    'editorBracketHighlight.foreground2': '#ffffff',
    'editorBracketHighlight.foreground3': '#ffffff',
    'editorBracketHighlight.foreground4': '#ffffff',
    'editorBracketHighlight.foreground5': '#ffffff',
    'editorBracketHighlight.foreground6': '#ffffff',
}"#;
