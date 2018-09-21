

macro_rules! my_macro {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            // This is purely an example—not a good one.
            fn get_field_names() -> Vec<&'static str> {
                vec![$(stringify!($field_name)),*]
            }
        }
    }
}

my_macro! {
    struct S {
        a: String,
        b: String,
    }
}