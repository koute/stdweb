use webcore::value::{Value, Reference, FromReferenceUnchecked};
use webcore::try_from::TryInto;
use webapi::file::File;

/// An object of this type is returned by the files property of the HTML `<input>` element;
/// this lets you access the list of files selected with the `<input type="file">` element.
/// It's also used for a list of files dropped into web content when using the drag and drop API.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileList)
// https://w3c.github.io/FileAPI/#dfn-filelist
pub struct FileList( Reference );

reference_boilerplate! {
    FileList,
    instanceof FileList
}

impl FileList {
    /// Returns the number of [File](struct.File.html)s contained in this list.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/FileList/length)
    // https://w3c.github.io/FileAPI/#ref-for-dfn-length
    pub fn len( &self ) -> usize {
        let length: i32 = js!( return @{self}.length; ).try_into().unwrap();
        length as usize
    }

    /// Returns an iterator over the list.
    pub fn iter( &self ) -> FileIter {
        FileIter {
            list: self.clone(),
            index: 0
        }
    }
}

impl IntoIterator for FileList {
    type Item = File;
    type IntoIter = FileIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        FileIter {
            list: self,
            index: 0
        }
    }
}

impl< 'a > IntoIterator for &'a FileList {
    type Item = File;
    type IntoIter = FileIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        FileIter {
            list: self.clone(),
            index: 0
        }
    }
}

#[derive(Debug)]
pub struct FileIter {
    list: FileList,
    index: i32
}

impl Iterator for FileIter {
    type Item = File;
    fn next( &mut self ) -> Option< Self::Item > {
        let value = js!(
            return @{&self.list}[ @{self.index} ];
        );

        let file = match value {
            Value::Undefined => return None,
            Value::Reference( reference ) => unsafe { File::from_reference_unchecked( reference ) },
            _ => unreachable!()
        };

        self.index += 1;
        Some( file )
    }
}
