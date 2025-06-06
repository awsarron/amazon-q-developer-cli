// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateCustomizationPermissionOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DisassociateCustomizationPermissionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateCustomizationPermissionOutput {
    /// Creates a new builder-style object to manufacture
    /// [`DisassociateCustomizationPermissionOutput`](crate::operation::disassociate_customization_permission::DisassociateCustomizationPermissionOutput).
    pub fn builder() -> crate::operation::disassociate_customization_permission::builders::DisassociateCustomizationPermissionOutputBuilder{
        crate::operation::disassociate_customization_permission::builders::DisassociateCustomizationPermissionOutputBuilder::default()
    }
}

/// A builder for
/// [`DisassociateCustomizationPermissionOutput`](crate::operation::disassociate_customization_permission::DisassociateCustomizationPermissionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DisassociateCustomizationPermissionOutputBuilder {
    _request_id: Option<String>,
}
impl DisassociateCustomizationPermissionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }

    /// Consumes the builder and constructs a
    /// [`DisassociateCustomizationPermissionOutput`](crate::operation::disassociate_customization_permission::DisassociateCustomizationPermissionOutput).
    pub fn build(
        self,
    ) -> crate::operation::disassociate_customization_permission::DisassociateCustomizationPermissionOutput {
        crate::operation::disassociate_customization_permission::DisassociateCustomizationPermissionOutput {
            _request_id: self._request_id,
        }
    }
}
