# A toy browser engine to learn Rust

### HTML PARSER

#### Allowed Syntax:

- Balanced tags: `<p>...</p>`
- Attributes with quoted values: `id="main"`
- Text nodes: `<em>world</em>`

#### To-Do:

- Comments
- Doctype declarations
- Escaped characters (like `&amp;`) and CDATA sections
- Self closing tags: `<br/>` or `<br>` with no closing tag
- Error handling (e.g. unbalanced or improperly nested tags)
- Namespaces and other XHTML syntax: `<html:body>`
- Character encoding detection
