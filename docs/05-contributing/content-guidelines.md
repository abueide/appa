# Content Guidelines

## Content Organization Principles

### Progressive Disclosure
- Start with simple concepts, build to complex
- Provide clear entry points for different user types
- Link to deeper explanations rather than overwhelming in one page

### User-Centered Approach
- Organize by what users want to accomplish, not internal code structure
- Lead with the "why" before the "how"
- Provide concrete examples and use cases

### Consistency
- Use the same terms throughout all documentation
- Follow the same structure patterns within each section
- Maintain consistent depth levels across similar topics

## Page Structure Templates

### Getting Started Pages
```markdown
# Page Title

Brief overview of what this page covers.

## Prerequisites
- What users need before starting

## Step-by-step Instructions
1. First step with explanation
2. Second step with example
3. Result validation

## Next Steps
- Link to related topics
- Suggested follow-up actions
```

### Reference Pages
```markdown
# Command/Feature Name

One-sentence description.

## Syntax
```bash
command syntax here
```

## Options
- `--option`: Description
- `--another`: Description

## Examples
Common use cases with explanations.

## See Also
- Related commands/concepts
```

### Architecture Pages
```markdown
# Concept/System Name

High-level overview and purpose.

## Design Goals
- Why this approach was chosen

## Implementation
- How it works

## Examples
- Real-world usage patterns

## Integration
- How it fits with other components
```

## Content Quality Checklist

- [ ] Clear page purpose in first paragraph
- [ ] Code examples that actually work
- [ ] Cross-references to related topics
- [ ] Examples use consistent naming (system names, etc.)
- [ ] No broken internal links
- [ ] Appropriate depth for target audience