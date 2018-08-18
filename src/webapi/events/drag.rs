#[cfg(feature = "futures-support")]
use futures_channel::oneshot;
use webapi::event::{IEvent, IUiEvent, Event, ConcreteEvent};
use webapi::events::mouse::{IMouseEvent, MouseEvent};
use webapi::file::File;
use webcore::once::Once;
use webcore::value::{Reference, Value};
use webcore::try_from::TryInto;
use webapi::file_list::FileList;
use webapi::html_elements::ImageElement;

/// The DragEvent interface is a DOM event that represents a drag and drop interaction.
/// The user initiates a drag by placing a pointer device (such as a mouse) on the touch surface
/// and then dragging the pointer to a new location (such as another DOM element).
///
/// This interface inherits properties from MouseEvent and Event.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent)
// https://www.w3.org/TR/html51/editing.html#the-dragevent-interface
pub trait IDragEvent: IMouseEvent {
    /// The DataEvent.dataTransfer property holds the drag operation's data (as a DataTransfer object).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent/dataTransfer)
    #[inline]
    fn data_transfer(&self) -> DataTransfer {
        js!(
            return @{self.as_ref()}.dataTransfer;
        ).try_into().unwrap()
    }
}

/// A reference to a JavaScript object which implements the [IDragEvent](trait.IDragEvent.html)
/// interface.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DragEvent)
// https://www.w3.org/TR/html51/editing.html#the-dragevent-interface
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DragEvent(Reference);

impl IEvent for DragEvent {}

impl IUiEvent for DragEvent {}

impl IMouseEvent for DragEvent {}

impl IDragEvent for DragEvent {}

/// The drag event is fired every few hundred milliseconds as an element or text selection is being
/// dragged by the user.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/drag)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-drag
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent, DragEvent))]
pub struct DragRelatedEvent(Reference);

impl IEvent for DragRelatedEvent {}

impl IUiEvent for DragRelatedEvent {}

impl IMouseEvent for DragRelatedEvent {}

impl IDragEvent for DragRelatedEvent {}

impl ConcreteEvent for DragRelatedEvent {
    const EVENT_TYPE: &'static str = "drag";
}

/// The dragstart event is fired when the user starts dragging an element or text selection.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dragstart)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-dragstart
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent, DragEvent))]
pub struct DragStartEvent(Reference);

impl IEvent for DragStartEvent {}

impl IUiEvent for DragStartEvent {}

impl IMouseEvent for DragStartEvent {}

impl IDragEvent for DragStartEvent {}

impl ConcreteEvent for DragStartEvent {
    const EVENT_TYPE: &'static str = "dragstart";
}

/// The dragenter event is fired when a dragged element or text selection enters a valid drop target.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dragenter)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-dragenter
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DragEnterEvent(Reference);

impl IEvent for DragEnterEvent {}

impl IUiEvent for DragEnterEvent {}

impl IMouseEvent for DragEnterEvent {}

impl IDragEvent for DragEnterEvent {}

impl ConcreteEvent for DragEnterEvent {
    const EVENT_TYPE: &'static str = "dragenter";
}

/// The dragstart event is fired when the user starts dragging an element or text selection.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dragexit)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-dragexit
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DragExitEvent(Reference);

impl IEvent for DragExitEvent {}

impl IUiEvent for DragExitEvent {}

impl IMouseEvent for DragExitEvent {}

impl IDragEvent for DragExitEvent {}

impl ConcreteEvent for DragExitEvent {
    const EVENT_TYPE: &'static str = "dragexit";
}

/// The dragleave event is fired when a dragged element or text selection leaves a valid drop target.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dragleave)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-dragleave
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DragLeaveEvent(Reference);

impl IEvent for DragLeaveEvent {}

impl IUiEvent for DragLeaveEvent {}

impl IMouseEvent for DragLeaveEvent {}

impl IDragEvent for DragLeaveEvent {}

impl ConcreteEvent for DragLeaveEvent {
    const EVENT_TYPE: &'static str = "dragleave";
}

/// The dragover event is fired when an element or text selection is being dragged over a valid drop
/// target (every few hundred milliseconds).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dragover)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-dragover
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DragOverEvent(Reference);

impl IEvent for DragOverEvent {}

impl IUiEvent for DragOverEvent {}

impl IMouseEvent for DragOverEvent {}

impl IDragEvent for DragOverEvent {}

impl ConcreteEvent for DragOverEvent {
    const EVENT_TYPE: &'static str = "dragover";
}

/// The dragstart event is fired when the user starts dragging an element or text selection.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/drop)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-drop
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DropEvent(Reference);

impl IEvent for DropEvent {}

impl IUiEvent for DropEvent {}

impl IMouseEvent for DropEvent {}

impl IDragEvent for DropEvent {}

impl ConcreteEvent for DropEvent {
    const EVENT_TYPE: &'static str = "drop";
}

/// The dragend event is fired when a drag operation is being ended (by releasing a mouse button or
/// hitting the escape key).
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/Events/dragend)
// https://www.w3.org/TR/html51/editing.html#eventdef-global-dragend
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DragEvent")]
#[reference(subclass_of(Event, MouseEvent))]
pub struct DragEndEvent(Reference);

impl IEvent for DragEndEvent {}

impl IUiEvent for DragEndEvent {}

impl IMouseEvent for DragEndEvent {}

impl IDragEvent for DragEndEvent {}

impl ConcreteEvent for DragEndEvent {
    const EVENT_TYPE: &'static str = "dragend";
}

/// The DataTransfer object is used to hold the data that is being dragged during a drag and drop
/// operation.
/// It may hold one or more data items, each of one or more data types.
/// For more information about drag and drop, see HTML Drag and Drop API.
///
/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)
// https://www.w3.org/TR/html51/editing.html#the-datatransfer-interface
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DataTransfer")]
pub struct DataTransfer( Reference );
impl DataTransfer {
    /// Gets the type of drag-and-drop operation currently selected or sets the operation to a new
    /// type. The value must be none, copy, link or move.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)
    pub fn drop_effect( &self ) -> DropEffect {
        let v: String = js!(
            return @{self.as_ref()}.dropEffect;
        ).try_into().unwrap();
        match v.as_ref() {
            "copy" => DropEffect::Copy,
            "move" => DropEffect::Move,
            "link" => DropEffect::Link,
            "none" => DropEffect::None,
            other => panic!("Expected valid dropEffect value, got {:?}", other),
        }
    }

    /// Sets the type of drag-and-drop operation currently selected or sets the operation to a new
    /// type. The value must be none, copy, link or move.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/dropEffect)
    pub fn set_drop_effect( &self, value: DropEffect ) {
        js! { @(no_return)
            @{self.as_ref()}.dropEffect = @{match value {
                DropEffect::Copy => "copy",
                DropEffect::Move => "move",
                DropEffect::Link => "link",
                DropEffect::None => "none",
            }};
        }
    }

    /// Provides all of the types of operations that are possible.
    /// Must be one of none, copy, copyLink, copyMove, link, linkMove, move, all or uninitialized.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)
    pub fn effect_allowed( &self ) -> EffectAllowed {
        let v: String = js!(
            return @{self.as_ref()}.effectAllowed;
        ).try_into().unwrap();
        match v.as_ref() {
            "none" => EffectAllowed::None,
            "copy" => EffectAllowed::Copy,
            "copyLink" => EffectAllowed::CopyLink,
            "copyMove" => EffectAllowed::CopyMove,
            "link" => EffectAllowed::Link,
            "linkMove" => EffectAllowed::LinkMove,
            "move" => EffectAllowed::Move,
            "all" => EffectAllowed::All,
            "uninitialized" => EffectAllowed::Uninitialized,
            other => panic!("Expected valid effectAllowed value, got {:?}", other),
        }
    }

    /// Sets the effect that is allowed for a drag operation.
    /// Must be one of none, copy, copyLink, copyMove, link, linkMove, move, all or uninitialized.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Element/effectAllowed)
    pub fn set_effect_allowed( &self, value: EffectAllowed ) {
        js! { @(no_return)
            @{self.as_ref()}.effectAllowed = @{match value {
            EffectAllowed::None => "none",
            EffectAllowed::Copy => "copy",
            EffectAllowed::CopyLink => "copyLink",
            EffectAllowed::CopyMove => "copyMove",
            EffectAllowed::Link => "link",
            EffectAllowed::LinkMove => "linkMove",
            EffectAllowed::Move => "move",
            EffectAllowed::All => "all",
            EffectAllowed::Uninitialized => "uninitialized",
            }};
        }
    }

    /// Gives a DataTransferItemList object which is a list of all of the drag data.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)
    pub fn items( &self ) -> DataTransferItemList {
        js!(
            return @{self.as_ref()}.items;
        ).try_into().unwrap()
    }

    /// Contains a list of all the local files available on the data transfer.
    /// If the drag operation doesn't involve dragging files, this property is an empty list.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)
    pub fn files( &self ) -> FileList {
        js!(
            return @{self.as_ref()}.files;
        ).try_into().unwrap()
    }

    /// An array of strings giving the formats that were set in the dragstart event.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)
    pub fn types( &self ) -> Vec<String> {
        js!(
            return @{self.as_ref()}.types;
        ).try_into().unwrap()
    }

    /// Remove the data associated with a given type. The type argument is optional.
    /// If the type is empty or not specified, the data associated with all types is removed.
    /// If data for the specified type does not exist, or the data transfer contains no data,
    /// this method will have no effect.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)
    pub fn clear_data( &self, format: Option<&str> ) {
        match format {
            None => js!(@(no_return) @{self.as_ref()}.clearData()),
            Some(x) => js!(@(no_return) @{self.as_ref()}.clearData(@{x}))
        };
    }

    /// Retrieves the data for a given type, or an empty string if data for that type does not exist
    /// or the data transfer contains no data.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)
    pub fn get_data( &self, format: &str ) -> String {
        js!(
            return @{self.as_ref()}.getData(@{format});
        ).try_into().unwrap()
    }

    /// Set the data for a given type.
    /// If data for the type does not exist, it is added at the end, such that the last item in the
    /// types list will be the new format.
    /// If data for the type already exists, the existing data is replaced in the same position.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)
    pub fn set_data( &self, format: &str, data: &str ) {
        js!(@(no_return)
            @{self.as_ref()}.setData(@{format}, @{data});
        );
    }

    /// Set the image to be used for dragging if a custom one is desired.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)
    pub fn set_drag_image( &self, img: &ImageElement, x_offset: u32, y_offset: u32 ) {
        js!(@(no_return)
            @{self.as_ref()}.setDragImage(@{img.as_ref()}, @{x_offset}, @{y_offset});
        );
    }
}

/// A DOMString representing the drag operation effect.
// https://www.w3.org/TR/html51/editing.html#dom-datatransfer-dropeffect
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropEffect {
    /// A copy of the source item is made at the new location
    Copy,
    /// An item is moved to a new location.
    Move,
    /// A link is established to the source at the new location.
    Link,
    /// The item may not be dropped.
    None,
}

/// A DOMString representing the drag operation that is allowed.
// https://www.w3.org/TR/html51/editing.html#dom-datatransfer-effectallowed
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectAllowed {
    /// The item may not be dropped.
    None,
    /// A copy of the source item may be made at the new location.
    Copy,
    /// A copy or link operation is permitted.
    CopyLink,
    /// A copy or move operation is permitted.
    CopyMove,
    /// A link may be established to the source at the new location.
    Link,
    /// A link or move operation is permitted.
    LinkMove,
    /// An item may be moved to a new location.
    Move,
    /// All operations are permitted.
    All,
    /// The default value when the effect has not been set, equivalent to all.
    Uninitialized,
}

/// The DataTransferItemList object is a list of DataTransferItem objects representing items being
/// dragged.
/// During a drag operation, each DragEvent has a dataTransfer property and that property is a
/// DataTransferItemList.
///
/// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList)
// https://www.w3.org/TR/html51/editing.html#the-datatransferitemlist-interface
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DataTransferItemList")]
pub struct DataTransferItemList( Reference );
impl DataTransferItemList {
    /// An unsigned long that is the number of drag items in the list.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/length)
    pub fn len( &self ) -> u64 {
        js!(
            return @{self.as_ref()}.length;
        ).try_into().unwrap()
    }

    /// Adds an item (either a File object or a string) to the drag item list and returns
    /// a DataTransferItem object for the new item.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)
    pub fn add( &self, item: DataTransferItemAdd ) -> Option<DataTransferItem> {
        match item {
            DataTransferItemAdd::File(file) => {
                js!(
                    return @{self.as_ref()}.add(@{file});
                ).try_into().unwrap()
            },
            DataTransferItemAdd::String(data, ty) => {
                js!(
                    return @{self.as_ref()}.add(@{data}, @{ty})
                ).try_into().unwrap()
            }
        }
    }

    /// Removes the drag item from the list at the given index.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/remove)
    pub fn remove( &self, index: u32 ) {
        js!(@(no_return) @{self.as_ref()}.remove(@{index}));
    }

    /// Removes all of the drag items from the list.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/clear)
    pub fn clear( &self ) {
        js!(@(no_return) @{self.as_ref()}.clear());
    }

    /// Getter that returns a DataTransferItem at the given index.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/DataTransferItem)
    pub fn index( &self, index: u32 ) -> Option<DataTransferItem> {
        let v: Value = js!(
            return @{self.as_ref()}[@{index}];
        );
        match v {
            Value::Reference(_) => Some(v.try_into().unwrap()),
            _ => None,
        }
    }

    /// Returns an iterator over the list
    pub fn iter( &self ) -> DataTransferItemIter {
        DataTransferItemIter {
            list: self.clone(),
            index: 0,
        }
    }
}

impl IntoIterator for DataTransferItemList {
    type Item = DataTransferItem;
    type IntoIter = DataTransferItemIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        DataTransferItemIter {
            list: self,
            index: 0
        }
    }
}

impl< 'a > IntoIterator for &'a DataTransferItemList {
    type Item = DataTransferItem;
    type IntoIter = DataTransferItemIter;

    #[inline]
    fn into_iter( self ) -> Self::IntoIter {
        DataTransferItemIter {
            list: self.clone(),
            index: 0
        }
    }
}

impl Iterator for DataTransferItemIter {
    type Item = DataTransferItem;

    fn next( &mut self ) -> Option< Self::Item > {
        let v = self.list.index(self.index);
        if v.is_some() {
            self.index += 1;
        }
        v
    }
}

#[derive(Debug)]
pub struct DataTransferItemIter {
    list: DataTransferItemList,
    index: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTransferItemAdd {
    File(File),
    String(String, String),
}

/// The DataTransferItem object represents one drag data item. During a drag operation, each drag
/// event has a dataTransfer property which contains a list of drag data items. Each item in the
/// list is a DataTransferItem object.
///
/// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem)
// https://www.w3.org/TR/html51/editing.html#the-datatransferitem-interface
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "DataTransferItem")]
pub struct DataTransferItem( Reference );

impl DataTransferItem {
    /// The kind of drag data item, string or file.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)
    pub fn kind( &self ) -> DataTransferItemKind {
        let kind: String = js!(
            return @{self.as_ref()}.kind;
        ).try_into().unwrap();
        match kind.as_ref() {
            "string" => DataTransferItemKind::String,
            "file" => DataTransferItemKind::File,
            other => panic!("Expected valid kind, got {:?}", other),
        }
    }

    /// The drag data item's type, typically a MIME type.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/type)
    pub fn ty( &self ) -> String {
        js!(
            return @{self.as_ref()}.type;
        ).try_into().unwrap()
    }

    /// Returns the File object associated with the drag data item
    /// (or null if the drag item is not a file)
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsFile)
    pub fn get_as_file( &self ) -> File {
        js!(
            return @{self.as_ref()}.getAsFile();
        ).try_into().unwrap()
    }

    /// Invokes the specified callback with the drag data item string as its argument.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)
    pub fn get_as_string<F>( &self, callback: F )
        where F: FnOnce(String) + 'static {
        js!(@(no_return)
            @{self.as_ref()}.getAsString(@{Once(callback)});
        );
    }

    /// Invokes the specified callback with the drag data item string as its argument.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/getAsString)
    #[cfg(feature = "futures-support")]
    pub fn get_as_string_future( &self ) -> oneshot::Receiver<String> {
        let (sender, receiver) = oneshot::channel();
        let callback = |s: String| {
            match sender.send(s) {
                Ok(_) => {},
                Err(_) => {},
            };
        };

        js!(@(no_return)
            @{self.as_ref()}.getAsString(@{Once(callback)});
        );

        receiver
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The kind of drag data item, string or file.
///
/// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItem/kind)
pub enum DataTransferItemKind {
    File,
    String,
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_drag_event() {
        let event: DragEvent = js!(
            return new DragEvent(
                @{DragStartEvent::EVENT_TYPE},
                {
                    dataTransfer: new DataTransfer()
                }
            );
        ).try_into().unwrap();

        // effects
        assert_eq!(event.event_type(), DragStartEvent::EVENT_TYPE);
        assert_eq!(event.data_transfer().effect_allowed(), EffectAllowed::None);
        assert_eq!(event.data_transfer().drop_effect(), DropEffect::None);
        event.data_transfer().set_effect_allowed(EffectAllowed::CopyMove);
        event.data_transfer().set_drop_effect(DropEffect::Copy);
        // TODO how to test? can only set these during ondragstart event triggered in browser
        // assert_eq!(event.data_transfer().effect_allowed(), EffectAllowed::CopyMove);
        // assert_eq!(event.data_transfer().drop_effect(), DropEffect::Copy);

        // get, set, clear data
        event.data_transfer().set_data("myformat", "mydata");
        event.data_transfer().set_data("myformat2", "mydata2");
        event.data_transfer().clear_data(Some("myformat3"));
        assert_eq!(event.data_transfer().get_data("myformat"), String::from("mydata"));
        event.data_transfer().clear_data(Some("myformat"));
        assert_eq!(event.data_transfer().get_data("myformat"), String::from(""));
        assert_eq!(event.data_transfer().get_data("myformat2"), String::from("mydata2"));
        event.data_transfer().clear_data(None);
        assert_eq!(event.data_transfer().get_data("myformat2"), String::from(""));
        let img = ImageElement::new();
        event.data_transfer().set_drag_image(&img, 10, 10);

        // types
        assert_eq!(event.data_transfer().types().len(), 0);

        // items
        assert_eq!(event.data_transfer().items().len(), 0);
        let data = "mydata";
        let ty = "text/plain";
        let item = event.data_transfer().items().add(
            DataTransferItemAdd::String(String::from(data), String::from(ty))
        ).unwrap();
        assert_eq!(item.ty(), ty);
        assert_eq!(item.kind(), DataTransferItemKind::String);
        // TODO(https://github.com/koute/stdweb/issues/128) fix when async unit testing is available
        // item.get_as_string().and_then(|s| {
        //     assert_eq!(data, s);
        //     assert_eq!(1, 1);
        //     assert_eq!(2, 1);
        // });
        let filename = "myname";
        let file = js!(return new File(["content"], @{filename})).try_into().unwrap();
        event.data_transfer().items().add(DataTransferItemAdd::File(file)).unwrap();
        assert_eq!(event.data_transfer().items().len(), 2);
        assert_eq!(event.data_transfer().items().iter().count(), 2);
        assert!(event.data_transfer().items().index(2).is_none());
        assert_eq!(event.data_transfer().files().len(), 1);
        let item = event.data_transfer().items().index(1).unwrap();
        assert_eq!(item.kind(), DataTransferItemKind::File);
        assert_eq!(item.get_as_file().name(), filename);
        event.data_transfer().items().remove(0);
        assert_eq!(event.data_transfer().items().len(), 1);
        event.data_transfer().items().clear();
        assert_eq!(event.data_transfer().items().len(), 0);
    }
}
