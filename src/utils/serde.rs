/*------------------------------------ DESERIALIZATION HELPERS -----------------------------------*/

use std::marker::PhantomData;

pub(crate) struct StructVisitor<T>(PhantomData<T>);

impl<T> StructVisitor<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData::<T>)
    }
}

// CHECK: if pub0, pub1, pub2 can be removed and just hard-coded as pub(crate)
macro_rules! setup_deserializable_structs {
    ( $(
        $pub0:vis $struct_name:ident {
            $( $pub1:vis $field:ident: $field_type:ty = $field_default:expr ;)*
            ---
            $( $pub2:vis $field_struct:ident: $field_struct_type:ident ;)*
        }
    ,)* ) => {
    use crate::utils::serde::StructVisitor;
    use ::serde::de::{Error, MapAccess, Visitor};
    use ::serde::{Deserialize, Deserializer};

    $(

        #[derive(Debug)]
        $pub0 struct $struct_name {
            $( $pub1 $field: $field_type ,)*
            $( $pub2 $field_struct: $field_struct_type ,)*
        }

        impl Default for $struct_name {
            fn default() -> Self {
                Self {
                    $( $field: $field_default ,)*
                    $( $field_struct: $field_struct_type::default() ,)*
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
                    $( $field_struct: $field_struct.unwrap_or_else($field_struct_type::default) ,)*
                })
            }
        }

    )* };
}

pub(crate) use setup_deserializable_structs;
