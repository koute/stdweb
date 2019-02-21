use webapi::blob::IBlob;
use webapi::dom_exception::InvalidStateError;
use webapi::element::IElement;
use webapi::error::TypeError;
use webapi::file::File;
use webcore::try_from::TryFrom;
use webcore::try_from::TryInto;
use webcore::value::ConversionError;
use webcore::value::Reference;
use webcore::value::Value;

/// The `FormData` interface provides a way to easily construct a set of key/value pairs
/// representing form fields and their values, which can then be easily sent using the
/// `XMLHttpRequest.send()` method. It uses the same format a form would use if the encoding type
/// were set to `"multipart/form-data"`.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData)
// https://xhr.spec.whatwg.org/#formdata
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "FormData")]
pub struct FormData( Reference );

/// Represents a type of data stores in FormData.
#[derive(Clone, Debug, PartialEq)]
pub enum FormDataEntry {
    /// File data
    File( File ),
    /// Text data
    String( String )
}

error_enum_boilerplate! {
    FormDataFromElementError,
    InvalidStateError, TypeError
}

impl TryFrom< Value > for FormDataEntry {
    type Error = ConversionError;

    fn try_from(value: Value) -> Result< Self, Self::Error > {
        let entry = match value {
            Value::String(s) => FormDataEntry::String(s),
            Value::Reference(r) => FormDataEntry::File(File(r)),
            _ => return Err(ConversionError::type_mismatch(&value, "string or reference".into())),
        };

        Ok(entry)
    }
}

impl TryFrom< Value > for Option< FormDataEntry > {
    type Error = ConversionError;

    fn try_from(value: Value) -> Result< Self, Self::Error > {
        let entry = match value {
            Value::Null|Value::Undefined => None,
            Value::String(s) => Some(FormDataEntry::String(s)),
            Value::Reference(r) => Some(FormDataEntry::File(File(r))),
            _ => return Err(ConversionError::type_mismatch(&value, "null, string or reference".into())),
        };

        Ok(entry)
    }
}

impl FormData {
    /// Creates a new `FormData`.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)
    pub fn new() -> Self {
        js! (
            return new FormData();
        ).try_into().unwrap()
    }

    /// Creates a new `FormData` from a form element.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)
    pub fn from_element<T>( form: &T ) -> Result< Self, FormDataFromElementError > where T: IElement {
        js_try! (
            let form = @{form.as_ref()};

            if ( ! (form instanceof HTMLFormElement) ) {
                throw new TypeError("Argument 1 of FormData::from_element does not implement interface HTMLFormElement.");
            }

            return new FormData(form);
        ).unwrap()
    }

    /// Appends a new value onto an existing key, or adds the key if it does not already exist.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)
    // https://xhr.spec.whatwg.org/#dom-formdata-append
    pub fn append_string( &self, name: &str, value: &str ) {
        js! { @(no_return)
            @{self}.append(@{name}, @{value});
        }
    }

    /// Appends a new blob onto an existing key, or adds the key if it does not already exist.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)
    // https://xhr.spec.whatwg.org/#dom-formdata-append-blob
    pub fn append_blob<T>( &self, name: &str, value: &T, filename: Option< &str > ) where T: IBlob {
        js! { @(no_return)
            @{self}.append(@{name}, @{value.as_ref()}, @{filename});
        }
    }

    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/delete)
    // https://xhr.spec.whatwg.org/#dom-formdata-delete
    pub fn delete( &self, name: &str ) {
        js! { @(no_return)
            @{self}.delete(@{name});
        }
    }

    /// Deletes a key and its value(s).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/get)
    // https://xhr.spec.whatwg.org/#dom-formdata-get
    pub fn get( &self, name: &str ) -> Option< FormDataEntry > {
        js! (
            return @{self}.get(@{name});
        ).try_into().unwrap()
    }

    /// Returns all the values associated with a given key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/getAll)
    // https://xhr.spec.whatwg.org/#dom-formdata-getall
    pub fn get_all( &self, name: &str ) -> Vec< FormDataEntry > {
        js! (
            return @{self}.getAll(@{name});
        ).try_into().unwrap()
    }

    /// Returns a boolean stating whether a `FormData` object contains a certain key.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/has)
    // https://xhr.spec.whatwg.org/#dom-formdata-has
    pub fn has( &self, name: &str ) -> bool {
        js! (
            return @{self}.has(@{name});
        ).try_into().unwrap()
    }

    /// Sets a new value for an existing key, or adds the key/value if it does not already exist.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)
    // https://xhr.spec.whatwg.org/#dom-formdata-set
    pub fn set_string( &self, name: &str, value: &str ) {
        js! { @(no_return)
            @{self}.set(@{name}, @{value});
        }
    }

    /// Sets a new blob for an existing key, or adds the key/value if it does not already exist.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)
    // https://xhr.spec.whatwg.org/#dom-formdata-set-blob
    pub fn set_blob<T>( &self, name: &str, value: &T, filename: Option< &str > ) where T: IBlob {
        js! { @(no_return)
            @{self}.set(@{name}, @{value.as_ref()}, @{filename});
        }
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use stdweb::webapi::blob::Blob;
    use stdweb::webapi::element::Element;
    use stdweb::webcore::try_from::TryInto;
    use super::*;

    fn form() -> Element {
        js!(
            let form = document.createElement("form");

            let inputs = [];

            for (let i = 1; i < 4; i++) {
                inputs[i] = document.createElement("input");
                inputs[i].name = "key" + i;
                inputs[i].value = "value" + i;
                form.appendChild(inputs[i]);
            }

            inputs[3].name = "key2";

            return form;
        ).try_into().unwrap()
    }

    fn data() -> FormData {
        let form = form();

        FormData::from_element(&form)
            .unwrap()
    }

    #[test]
    fn test_new() {
        FormData::new();
    }

    #[test]
    fn test_from_invalid_element() {
        use webapi::document::document;

        let div = document().create_element("div")
            .unwrap();

        assert!(FormData::from_element(&div).is_err());
    }

    #[test]
    fn test_append_string() {
        let data = data();
        assert!(data.get("key0").is_none());

        data.append_string("key0", "value0");
        assert_eq!(data.get("key0"), Some(FormDataEntry::String(String::from("value0"))));
    }

    #[test]
    fn test_append_blob() {
        let data = data();
        assert!(data.get("key0").is_none());

        data.append_blob("blob", &Blob::new(), Some("file.jpg"));
        assert!(data.get("blob").is_some());
    }

    #[test]
    fn test_delete() {
        let data = data();
        assert!(data.get("key1").is_some());

        data.delete("key1");
        assert!(data.get("key1").is_none());
    }

    #[test]
    fn test_get() {
        let data = data();

        assert_eq!(data.get("key1"), Some(FormDataEntry::String(String::from("value1"))));
    }

    #[test]
    fn test_get_all() {
        let data = data();

        assert_eq!(data.get_all("key2"), vec![
            FormDataEntry::String(String::from("value2")),
            FormDataEntry::String(String::from("value3"))
        ]);
        assert_eq!(data.get_all("unknow"), Vec::<FormDataEntry>::new());
    }

    #[test]
    fn test_has() {
        let data = data();

        assert_eq!(data.has("key1"), true);
    }

    #[test]
    fn test_set_string() {
        let data = data();
        assert_eq!(data.get("key1"), Some(FormDataEntry::String(String::from("value1"))));

        data.set_string("key1", "value");
        assert_eq!(data.get("key1"), Some(FormDataEntry::String(String::from("value"))));
    }

    #[test]
    fn test_set_blob() {
        let data = data();
        assert_eq!(data.get("key1"), Some(FormDataEntry::String(String::from("value1"))));

        data.set_blob("key1", &Blob::new(), None);
        assert!(data.get("key1").is_some());
    }
}
