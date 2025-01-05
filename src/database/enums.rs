/// A macro helper to implement the `ToSql` and `FromSql` traits for an enum.
/// Unfortunately diesel doesn't automatically generate these for enums, so we
/// have to do it manually. This means we need to make sure that this enum
/// matches the definition in the database.
macro_rules! impl_enum {
    ($enum:ident, $sql_type:ty, {
        $(
            $variant:ident => $value:literal
        ),*$(,)?
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, ::diesel::deserialize::FromSqlRow, ::diesel::expression::AsExpression)]
        #[diesel(sql_type = $sql_type)]
        #[serde(rename_all = "snake_case")]
        pub enum $enum {
            $(
                $variant,
            )*
        }

        const _: () = {
            impl ::diesel::serialize::ToSql<$sql_type, ::diesel::pg::Pg> for $enum {
                fn to_sql<'b>(&'b self, out: &mut ::diesel::serialize::Output<'b, '_, ::diesel::pg::Pg>) -> ::diesel::serialize::Result {
                    match self {
                        $(
                            $enum::$variant => ::std::io::Write::write_all(out, $value)?,
                        )*
                    };

                    Ok(::diesel::serialize::IsNull::No)
                }
            }

            impl ::diesel::deserialize::FromSql<$sql_type, ::diesel::pg::Pg> for $enum {
                fn from_sql(bytes: <::diesel::pg::Pg as ::diesel::backend::Backend>::RawValue<'_>) -> ::diesel::deserialize::Result<Self> {
                    match bytes.as_bytes() {
                        $(
                            $value => Ok($enum::$variant),
                        )*
                        bytes => Err(format!("invalid {}: {:?}", stringify!($enum), bytes).into()),
                    }
                }
            }
        };
    };
}

impl_enum!(ApplicationStatus, super::schema::sql_types::ApplicationStatus, {
    Pending => b"pending",
    Approved => b"approved",
    Maybe => b"maybe",
    Rejected => b"rejected",
});

impl_enum!(TwitchAccountType, super::schema::sql_types::TwitchAccountType, {
    Pleb => b"pleb",
    Affiliate => b"affiliate",
    Partner => b"partner",
});
