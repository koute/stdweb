
/// Represents CORS (Cross Origin Resource Sharing) setting for an HTML element.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/HTML/CORS_settings_attributes)
// https://html.spec.whatwg.org/#cors-settings-attribute
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CrossOriginSetting {
    /// CORS is not used for this element.
    None,

    /// CORS requests for this element will not have the credentials flag set.
    Anonymous,

    /// CORS requests for this element will have the credentials flag set;
    /// this means the request will provide credentials.
    UseCredentials,
}
