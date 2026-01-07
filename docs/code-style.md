# Code Style Guide

## Core Principles

1. **Self-Documenting Code**: Code should read like prose
2. **Comments Explain Why**: Never what or how
3. **No Abbreviations**: Use full words
4. **Refactor Over Comment**: If you need to explain code, refactor it
5. **Never Nester**: Return early, avoid deep nesting
6. **Functional Core, Imperative Shell**: Pure logic inside, side effects at edges

## Comments Policy

### The Golden Rule

**Comments explain WHY, never WHAT or HOW.**

If you find yourself writing "what" or "how" comments, the code needs refactoring.

### Examples: Bad Comments

```rust
// ❌ WHAT comment - redundant
// Create a new vector
let items = Vec::new();

// ❌ HOW comment - code shows this
// Loop through columns and find the selected one
for (index, column) in columns.iter().enumerate() {
    if index == selected_column {
        return Some(column);
    }
}

// ❌ Explaining complex logic
// Check if task index is valid and get mutable reference
if task_idx < self.columns[self.selected_column].tasks.len() {
    let task = &mut self.columns[self.selected_column].tasks[task_idx];
    task.title = new_title;
}
```

### Examples: Good Refactoring

Instead of commenting, refactor for clarity:

```rust
// Before: Needs comment
// Check if the user has completed onboarding
if user.flags & 0x04 != 0 && user.last_login > cutoff_date {
    show_dashboard();
}

// After: Self-documenting
const ONBOARDING_COMPLETE_FLAG: u8 = 0x04;
let has_completed_onboarding = user.flags & ONBOARDING_COMPLETE_FLAG != 0;
let has_logged_in_recently = user.last_login > cutoff_date;

if has_completed_onboarding && has_logged_in_recently {
    show_dashboard();
}
```

```rust
// Before: Complex nested access
let task = &mut self.columns[self.selected_column].tasks[task_idx];

// After: Extract to method
let task = self.get_selected_task_mut()?;
```

### When WHY Comments Are Good

```rust
// ✅ Business logic rationale
// We always move to "In Progress" first to maintain audit history
// even if the task is immediately completed
task.move_to_column("In Progress");
task.move_to_column("Done");

// ✅ Performance trade-off
// Cache parsed templates to avoid parsing on every request.
// Memory usage: ~50KB per template, max 100 templates = 5MB
let template = template_cache.get_or_insert(path, parse_template);

// ✅ Non-obvious algorithm choice
// Binary search requires O(log n) but needs sorted data.
// We sort once at startup (O(n log n)) then search many times.
items.sort_unstable();
items.binary_search(&target);

// ✅ Workaround for external issue
// Clippy false positive: this pattern is safe but triggers warning
// See: https://github.com/rust-lang/rust-clippy/issues/12345
#[allow(clippy::some_lint)]

// ✅ Safety invariant documentation
// INVARIANT: buffer must be at least 1024 bytes
// This is guaranteed by the allocation in new()
let slice = &buffer[0..1024];
```

### Documentation Comments (///)

Doc comments are different - they document public APIs:

```rust
/// Parse a task from JSON string.
///
/// # Errors
///
/// Returns `ParseError` if JSON is malformed or missing required fields.
///
/// # Example
///
/// ```
/// let task = parse_task(r#"{"title": "Test"}"#)?;
/// ```
pub fn parse_task(json: &str) -> Result<Task> {
    // Implementation
}
```

## No Abbreviations

### The Rule

**Use full words for all identifiers. No abbreviations.**

This applies to:
- Variable names
- Function names
- Struct/enum names
- Field names
- Module names

### Why?

1. **Clarity**: `configuration` is clearer than `cfg`
2. **No ambiguity**: `msg` could mean message or messages
3. **No typing benefit**: IDEs autocomplete
4. **Universal understanding**: Non-native speakers understand full words
5. **Consistency**: No debates about which abbreviations to use

### Common Violations

| ❌ Don't Use | ✅ Use Instead |
|-------------|---------------|
| `cfg` | `config` or `configuration` |
| `ctx` | `context` |
| `msg` | `message` |
| `btn` | `button` |
| `idx` | `index` |
| `val` | `value` |
| `tmp` | `temporary` |
| `num` | `number` or `count` |
| `arr` | `array` |
| `doc` | `document` |
| `img` | `image` |
| `str` | `string` (except `&str` type) |
| `auth` | `authentication` |
| `info` | `information` |
| `calc` | `calculate` |
| `init` | `initialize` |
| `exec` | `execute` |
| `resp` | `response` |
| `req` | `request` |

### Allowed Exceptions

Only industry-standard terms where the abbreviation is MORE common:

- `id` - universally understood as "identifier"
- `url` - more common than "uniform resource locator"
- `html`, `css`, `json`, `xml` - file format names
- `io` - Rust standard library convention
- `uuid` - standard acronym
- `pty` - standard Unix pseudo-terminal term
- `api` - standard "application programming interface"

**Test**: If you need to explain the abbreviation, use the full word.

### Examples: Bad

```rust
// ❌ Too many abbreviations
fn proc_msg(msg: &str, cfg: &Config) -> Result<()> {
    let mut buf = Vec::new();
    let ctx = parse_ctx(msg)?;

    for itm in ctx.items {
        buf.push(fmt_itm(itm, cfg));
    }

    Ok(())
}
```

### Examples: Good

```rust
// ✅ Clear and readable
fn process_message(message: &str, config: &Config) -> Result<()> {
    let mut buffer = Vec::new();
    let context = parse_context(message)?;

    for item in context.items {
        buffer.push(format_item(item, config));
    }

    Ok(())
}
```

## Refactoring Triggers

If you find yourself:
1. **Adding "what" or "how" comments** → Extract to named function
2. **Using abbreviations to fit line length** → Extract to function
3. **Nesting more than 3 levels** → Extract to function
4. **Having functions >50 lines** → Split into smaller functions
5. **Repeating logic** → Extract to function
6. **Using complex boolean expressions** → Extract to named variable

### Example: Refactoring Nested Logic

```rust
// ❌ Before: Needs comments to understand
fn process_task(task: &Task) {
    // Check if task is valid
    if task.title.len() > 0 {
        // Check if task is not too old
        if task.created_at > Utc::now() - Duration::days(30) {
            // Check if task has no assignee
            if task.assignee.is_none() {
                assign_to_default_user(task);
            }
        }
    }
}

// ✅ After: Self-documenting
fn process_task(task: &Task) {
    if should_assign_to_default(task) {
        assign_to_default_user(task);
    }
}

fn should_assign_to_default(task: &Task) -> bool {
    task.has_title()
        && task.is_recent()
        && task.assignee.is_none()
}

impl Task {
    fn has_title(&self) -> bool {
        !self.title.is_empty()
    }

    fn is_recent(&self) -> bool {
        const MAX_AGE_DAYS: i64 = 30;
        self.created_at > Utc::now() - Duration::days(MAX_AGE_DAYS)
    }
}
```

## Never Nester: Return Early

**Avoid deep nesting by returning early. Keep the "happy path" at the lowest indentation level.**

### The Problem

```rust
// ❌ BAD: Deep nesting, hard to follow
fn process_task(task: Option<Task>) -> Result<()> {
    if let Some(task) = task {
        if task.is_valid() {
            if let Some(assignee) = task.assignee {
                if assignee.is_active() {
                    assignee.assign(task);
                    Ok(())
                } else {
                    Err("Assignee inactive")
                }
            } else {
                Err("No assignee")
            }
        } else {
            Err("Invalid task")
        }
    } else {
        Err("No task")
    }
}
```

### The Solution: Guard Clauses

```rust
// ✅ GOOD: Early returns, happy path clear
fn process_task(task: Option<Task>) -> Result<()> {
    let task = task.ok_or("No task")?;

    if !task.is_valid() {
        return Err("Invalid task".into());
    }

    let assignee = task.assignee.ok_or("No assignee")?;

    if !assignee.is_active() {
        return Err("Assignee inactive".into());
    }

    assignee.assign(task);
    Ok(())
}
```

### Rules

1. **Check errors first, return early**
2. **Extract valid data progressively**
3. **Keep the happy path at the lowest indentation**
4. **Use `?` operator for error propagation**
5. **Maximum nesting depth: 2 levels**

### More Examples

```rust
// ❌ BAD: Nested conditions
fn save_file(path: &Path, data: &[u8]) -> Result<()> {
    if path.exists() {
        if path.is_file() {
            if has_write_permission(path) {
                std::fs::write(path, data)?;
                Ok(())
            } else {
                Err("No write permission")
            }
        } else {
            Err("Not a file")
        }
    } else {
        Err("Path does not exist")
    }
}

// ✅ GOOD: Guard clauses
fn save_file(path: &Path, data: &[u8]) -> Result<()> {
    if !path.exists() {
        return Err("Path does not exist".into());
    }

    if !path.is_file() {
        return Err("Not a file".into());
    }

    if !has_write_permission(path) {
        return Err("No write permission".into());
    }

    std::fs::write(path, data)?;
    Ok(())
}
```

### Pattern: Replace Nested if/else with Early Returns

```rust
// ❌ Before
fn calculate_discount(user: &User, amount: f64) -> f64 {
    if user.is_premium() {
        if amount > 100.0 {
            amount * 0.2
        } else {
            amount * 0.1
        }
    } else {
        if amount > 100.0 {
            amount * 0.05
        } else {
            0.0
        }
    }
}

// ✅ After
fn calculate_discount(user: &User, amount: f64) -> f64 {
    if !user.is_premium() && amount <= 100.0 {
        return 0.0;
    }

    if !user.is_premium() {
        return amount * 0.05;
    }

    if amount > 100.0 {
        return amount * 0.2;
    }

    amount * 0.1
}
```

## Functional Core, Imperative Shell

**Separate pure logic (functional core) from side effects (imperative shell).**

This architecture makes code:
- Easier to test (pure functions need no mocks)
- Easier to reason about (no hidden state changes)
- More reusable (pure functions work anywhere)
- More maintainable (clear separation of concerns)

### The Pattern

```
┌─────────────────────────────────────┐
│     IMPERATIVE SHELL (thin)         │
│  - Read input                       │
│  - Call functional core             │
│  - Write output                     │
│  - Handle side effects              │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│     FUNCTIONAL CORE (thick)         │
│  - Pure functions                   │
│  - Business logic                   │
│  - No I/O                           │
│  - No mutation                      │
│  - Fully testable                   │
└─────────────────────────────────────┘
```

### Example: Task Processing

```rust
// ❌ BAD: Logic mixed with side effects
fn process_and_save_task(task_id: Uuid) -> Result<()> {
    let task = database.load_task(task_id)?;  // I/O

    if task.is_complete() {
        task.status = Status::Done;             // Mutation
        task.completed_at = Some(Utc::now());   // Side effect
        database.save(task)?;                   // I/O
        email.send_notification(task.assignee)?; // I/O
    }

    Ok(())
}

// ✅ GOOD: Functional core + imperative shell
// FUNCTIONAL CORE: Pure business logic
fn compute_task_updates(task: Task) -> Option<TaskUpdate> {
    if !task.is_complete() {
        return None;
    }

    Some(TaskUpdate {
        status: Status::Done,
        completed_at: Utc::now(),
        notify_assignee: true,
    })
}

// IMPERATIVE SHELL: Handle side effects
fn process_and_save_task(task_id: Uuid) -> Result<()> {
    let task = database.load_task(task_id)?;

    let Some(update) = compute_task_updates(task) else {
        return Ok(());
    };

    database.apply_update(task_id, &update)?;

    if update.notify_assignee {
        email.send_notification(task.assignee)?;
    }

    Ok(())
}
```

### Benefits

**Functional Core:**
```rust
// ✅ Easy to test - no mocks needed
#[test]
fn test_task_updates() {
    let task = Task {
        is_complete: true,
        ..Default::default()
    };

    let update = compute_task_updates(task);
    assert_eq!(update.status, Status::Done);
}
```

**vs. Imperative Mixed:**
```rust
// ❌ Hard to test - needs database mock, email mock
#[test]
fn test_process_task() {
    let mut mock_db = MockDatabase::new();
    let mut mock_email = MockEmail::new();
    // ... complex setup
}
```

### Rules for Separation

**Functional Core (Pure Functions):**
- ✅ Take data as parameters
- ✅ Return computed results
- ✅ No I/O (files, network, database)
- ✅ No mutation of external state
- ✅ No `Utc::now()` or random numbers (pass as parameter)
- ✅ Deterministic: same input → same output

**Imperative Shell:**
- ✅ Handle I/O operations
- ✅ Manage state changes
- ✅ Call pure functions
- ✅ Keep thin - delegate logic to core
- ✅ Coordinate side effects

### More Examples

```rust
// ❌ BAD: Side effects mixed in
fn validate_and_save_user(form: UserForm) -> Result<()> {
    if form.email.is_empty() {
        log::error!("Invalid email");  // Side effect
        return Err("Email required");
    }

    let user = User {
        email: form.email.to_lowercase(),  // Logic
        created_at: Utc::now(),            // Side effect
    };

    database.insert(user)?;                // Side effect
    analytics.track_signup()?;            // Side effect
    Ok(())
}

// ✅ GOOD: Separated
// FUNCTIONAL CORE
fn validate_user_form(form: &UserForm) -> Result<ValidatedUser> {
    if form.email.is_empty() {
        return Err("Email required".into());
    }

    Ok(ValidatedUser {
        email: form.email.to_lowercase(),
    })
}

fn create_user(validated: ValidatedUser, timestamp: DateTime<Utc>) -> User {
    User {
        email: validated.email,
        created_at: timestamp,
    }
}

// IMPERATIVE SHELL
fn validate_and_save_user(form: UserForm) -> Result<()> {
    let validated = validate_user_form(&form).map_err(|e| {
        log::error!("Validation failed: {e}");
        e
    })?;

    let user = create_user(validated, Utc::now());

    database.insert(user)?;
    analytics.track_signup()?;
    Ok(())
}
```

### Testing Benefits

```rust
// Functional core: Easy unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let form = UserForm { email: "".into() };
        assert!(validate_user_form(&form).is_err());
    }

    #[test]
    fn test_user_creation() {
        let validated = ValidatedUser { email: "test@example.com".into() };
        let timestamp = Utc::now();
        let user = create_user(validated, timestamp);
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.created_at, timestamp);
    }
}
```

## Code Review Checklist

Before submitting code, verify:

- [ ] No "what" or "how" comments (only "why")
- [ ] No abbreviations in identifiers
- [ ] All functions have clear, descriptive names
- [ ] Complex logic extracted to named functions
- [ ] Magic numbers replaced with named constants
- [ ] Boolean expressions named for clarity
- [ ] **Early returns used, maximum 2 levels of nesting**
- [ ] **Pure functions separated from side effects**
- [ ] Functions under 50 lines
- [ ] No code repetition

## Enforcement

These rules are checked during:
- Code review (human)
- Clippy lints where possible
- Team discussions

Remember:
- **Clarity over cleverness. Readability over brevity.**
- **Return early, nest never.**
- **Pure core, impure shell.**
