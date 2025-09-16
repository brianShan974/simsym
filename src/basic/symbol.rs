use derive_more::{Constructor, Display};

/// Creates a symbol given a `name`` and a `subscript`. The `name` must be a single `char` to avoid
/// using long variable names, but can be differentiated by the `subscript`. The `subscript` is an
/// `Option<usize>`. For example, a `Symbol` with name `x` and subscript `None` will be displayed
/// as `x`, and a `Symbol` with name `y` and subscript `32` will be displayed as `y_32`.
#[derive(Display, Debug, Clone, PartialEq, Constructor)]
#[display("{name}{}", subscript.map_or("".to_string(), |n| format!("_{n}")))]
pub struct Symbol {
    name: char,
    subscript: Option<usize>,
}

impl Symbol {
    /// Creates a `Symbol` with name only.
    pub fn with_name(name: char) -> Self {
        Self {
            name,
            subscript: None,
        }
    }

    pub fn with_name_and_subscript(name: char, subscript: usize) -> Self {
        Self {
            name,
            subscript: Some(subscript),
        }
    }
}

impl Default for Symbol {
    /// The default name is `x`, and the default subscript is `None`.
    fn default() -> Self {
        Self {
            name: 'x',
            subscript: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::symbol::Symbol;

    #[test]
    fn test_symbol_display() {
        let symbol_x = Symbol::default();
        assert_eq!(format!("{symbol_x}"), "x".to_string());

        let symbol_y = Symbol::with_name('y');
        assert_eq!(format!("{symbol_y}"), "y".to_string());

        let symbol_z0 = Symbol::new('z', Some(0));
        assert_eq!(format!("{symbol_z0}"), "z_0".to_string());

        let symbol_w32 = Symbol::with_name_and_subscript('w', 32);
        assert_eq!(format!("{symbol_w32}"), "w_32".to_string());
    }
}
