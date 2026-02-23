#[macro_export]
macro_rules! impl_element {
    // Variant 1: Fixed location (Location::Internal)
    ($struct:ident, $type_variant:expr, fixed) => {
        impl $crate::c4::Element for $struct {
            fn id(&self) -> &$crate::c4::ElementId {
                &self.id
            }
            fn name(&self) -> &str {
                &self.name
            }
            fn description(&self) -> &str {
                &self.description
            }
            fn element_type(&self) -> $crate::c4::ElementType {
                $type_variant
            }
            fn location(&self) -> $crate::c4::Location {
                $crate::c4::Location::Internal
            }
        }
        impl $struct {
            pub fn id(&self) -> &$crate::c4::ElementId {
                &self.id
            }
            pub fn name(&self) -> &str {
                &self.name
            }
            pub fn description(&self) -> &str {
                &self.description
            }
            pub fn location(&self) -> $crate::c4::Location {
                $crate::c4::Location::Internal
            }
        }
    };
    // Variant 2: Optional location (from self.location field)
    ($struct:ident, $type_variant:expr, optional) => {
        impl $crate::c4::Element for $struct {
            fn id(&self) -> &$crate::c4::ElementId {
                &self.id
            }
            fn name(&self) -> &str {
                &self.name
            }
            fn description(&self) -> &str {
                &self.description
            }
            fn element_type(&self) -> $crate::c4::ElementType {
                $type_variant
            }
            fn location(&self) -> $crate::c4::Location {
                self.location
                    .clone()
                    .unwrap_or($crate::c4::Location::Internal)
            }
        }
        impl $struct {
            pub fn id(&self) -> &$crate::c4::ElementId {
                &self.id
            }
            pub fn name(&self) -> &str {
                &self.name
            }
            pub fn description(&self) -> &str {
                &self.description
            }
            pub fn location(&self) -> $crate::c4::Location {
                self.location
                    .clone()
                    .unwrap_or($crate::c4::Location::Internal)
            }
        }
    };
    // Default: fixed location (backward compatible)
    ($struct:ident, $type_variant:expr) => {
        $crate::impl_element!($struct, $type_variant, fixed);
    };
}

pub use impl_element;
