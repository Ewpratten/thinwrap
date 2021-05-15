pub use std::ops::{Deref, DerefMut};

/// `auto_deref` simply fills in `Deref` and `DerefMut` for any struct
#[macro_export]
macro_rules! auto_deref {
    ($outer: ident, $inner: ty) => {
        impl Deref for $outer {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $outer {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// `thin_wrap` is a macro for creating a new struct that directly derefs into an inner struct.
#[macro_export]
macro_rules! thin_wrap {

    // Just two names
    ($outer: ident, $inner: ty) => {
        thin_wrap!(pub(self), $outer, $inner);
    };

    // Core / "everything" macro
    ($v:vis, $outer: ident, $inner: ty) => {
        $v struct $outer($inner);

        auto_deref!($outer, $inner);
    };
}

#[cfg(test)]
mod tests {

    pub use super::*;

    // Test inner struct
    struct Inner {
        pub x: u8,
    }

    thin_wrap!(Outer, Inner);

    #[test]
    fn test_outer_struct() {
        let o = Outer { 0: Inner { x: 15 } };

        assert_eq!(15, o.x);
    }
}
