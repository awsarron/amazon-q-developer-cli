// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ChangeLogOptions {
    #[allow(missing_docs)] // documentation missing in model
    pub granularity: crate::types::ChangeLogGranularityType,
}
impl ChangeLogOptions {
    #[allow(missing_docs)] // documentation missing in model
    pub fn granularity(&self) -> &crate::types::ChangeLogGranularityType {
        &self.granularity
    }
}
impl ChangeLogOptions {
    /// Creates a new builder-style object to manufacture
    /// [`ChangeLogOptions`](crate::types::ChangeLogOptions).
    pub fn builder() -> crate::types::builders::ChangeLogOptionsBuilder {
        crate::types::builders::ChangeLogOptionsBuilder::default()
    }
}

/// A builder for [`ChangeLogOptions`](crate::types::ChangeLogOptions).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ChangeLogOptionsBuilder {
    pub(crate) granularity: ::std::option::Option<crate::types::ChangeLogGranularityType>,
}
impl ChangeLogOptionsBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn granularity(mut self, input: crate::types::ChangeLogGranularityType) -> Self {
        self.granularity = ::std::option::Option::Some(input);
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn set_granularity(mut self, input: ::std::option::Option<crate::types::ChangeLogGranularityType>) -> Self {
        self.granularity = input;
        self
    }

    #[allow(missing_docs)] // documentation missing in model
    pub fn get_granularity(&self) -> &::std::option::Option<crate::types::ChangeLogGranularityType> {
        &self.granularity
    }

    /// Consumes the builder and constructs a [`ChangeLogOptions`](crate::types::ChangeLogOptions).
    /// This method will fail if any of the following fields are not set:
    /// - [`granularity`](crate::types::builders::ChangeLogOptionsBuilder::granularity)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::ChangeLogOptions, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ChangeLogOptions {
            granularity: self.granularity.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "granularity",
                    "granularity was not specified but it is required when building ChangeLogOptions",
                )
            })?,
        })
    }
}
