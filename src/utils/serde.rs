/*------------------------------------ DESERIALIZATION HELPERS -----------------------------------*/

use std::marker::PhantomData;

pub(crate) struct StructVisitor<T>(PhantomData<T>);

impl<T> StructVisitor<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData::<T>)
    }
}

macro_rules! setup_deserializable_structs {
    ( $(
        $pub:vis $struct_name:ident {
            $( $field:ident: $field_type:ty = $field_default:expr ;)*
            ---
            $( $field_struct:ident: $field_struct_type:ident ;)*
        }
    ,)* ) => {
    use crate::utils::serde::StructVisitor;

    $(

        #[derive(Debug)]
        $pub struct $struct_name {
            $( $field: $field_type ,)*
            $( $field_struct: $field_struct_type ,)*
        }

        impl $struct_name {
            $pub const fn new() -> Self {
                Self {
                    $( $field: $field_default ,)*
                    $( $field_struct: $field_struct_type::new() ,)*
                }
            }
        }

        impl<'de> Deserialize<'de> for $struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>
            {
                deserializer.deserialize_any(StructVisitor::<$struct_name>::new())
            }
        }

        impl<'de> Visitor<'de> for StructVisitor<$struct_name> {
            type Value = $struct_name;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(format!("a `{}` table.", stringify!($struct_name)).as_str())
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>
            {
                $( let mut $field = None; )*
                $( let mut $field_struct = None; )*

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        $( stringify!($field) => $field = Some(map.next_value()?) ,)*
                        $( stringify!($field_struct) => $field_struct = Some(map.next_value()?) ,)*
                        _ => return Err(M::Error::unknown_field(
                                &key,
                                &[$( stringify!($field) ,)* $( stringify!($field_struct) ,)*])
                            ),
                    }
                }

                Ok(Self::Value {
                    $( $field: $field.unwrap_or($field_default) ,)*
                    $( $field_struct: $field_struct.unwrap_or_else($field_struct_type::new) ,)*
                })
            }
        }

    )* };
}

pub(crate) use setup_deserializable_structs;
