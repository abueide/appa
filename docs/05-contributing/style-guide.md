# Wiki Style Guide

## Writing Style

### Tone
- **Concise and direct** - Get to the point quickly
- **Helpful, not prescriptive** - Guide users, don't dictate
- **Technical but accessible** - Assume basic technical knowledge

### Voice
- Use active voice when possible
- Write in second person ("you") for user-facing content
- Use present tense for current functionality

## Formatting Conventions

### Headers
- Use sentence case for headers (capitalize first word only)
- Start with H1 (`#`) for page title
- Use H2 (`##`) for main sections
- Use H3 (`###`) for subsections
- Avoid going deeper than H4

### Code and Commands
- Use backticks for inline code: `appa show system-01`
- Use code blocks with language specification:
  ```yaml
  name: web-server
  type: vm
  ```
- Use `bash` for command examples with `$` prompt:
  ```bash
  $ appa add system web-01 --type=server
  ```

### Links
- Use descriptive link text (not "click here" or bare URLs)
- Link to specific sections when helpful: `[Systems](../01-user-guide/managing-systems.md#adding-systems)`
- Use relative paths for internal links
- Open external links in same tab (standard markdown)

### Lists
- Use `-` for unordered lists
- Use `1.` for ordered lists (auto-numbering)
- Use consistent indentation (2 spaces)

### Emphasis
- Use **bold** for UI elements, important concepts, and file names
- Use *italics* for emphasis and first use of terms
- Use `code formatting` for commands, file paths, and code elements

## File Naming

- Use lowercase with hyphens: `managing-systems.md`
- Be descriptive but concise
- Use `README.md` for directory index pages

## Cross-References

- Link liberally to related concepts
- Add "See also" sections when helpful
- Use consistent terminology across all docs