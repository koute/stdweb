use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::node::{INode, Node};
use webapi::dom_exception::{IndexSizeError, NotFoundError, InvalidStateError};

/// Possible values are:
///
/// * `None`: No selection has currently been made.
/// * `Caret`: The selection is collapsed (i.e. the caret is placed on some text, but no
/// range has been selected).
/// * `Range`: A range has been selected.
///
/// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SelectionType {
    None,
    Caret,
    Range
}

/// Represents the range of text selected by the user or the current position of the caret. To
/// obtain a Selection object for examination or modification, call
/// [Window.get_selection()](struct.Window.html#method.get_selection).
///
/// A user may make a selection from left to right (in document order) or right to left (reverse of
/// document order). The anchor is where the user began the selection and the focus is where the
/// user ends the selection. If you make a selection with a desktop mouse, the anchor is placed
/// where you pressed the mouse button and the focus is placed where you released the mouse button.
/// Anchor and focus should not be confused with the start and end positions of a selection, since
/// anchor can be placed before the focus or vice versa, depending on the direction you made your
/// selection.
///
/// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection)
// https://w3c.github.io/selection-api/#selection-interface
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Selection")]
pub struct Selection(Reference);

impl Selection {
    /// Returns the [Node](struct.Node.html) in which the selection begins.
    ///
    /// A user may make a selection from left to right (in document order) or right to left
    /// (reverse of document order). The anchor is where the user began the selection. This can be
    /// visualized by holding the Shift key and pressing the arrow keys on your keyboard. The
    /// selection's anchor does not move, but the selection's focus, the other end of the
    /// selection, does move.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorNode)
    pub fn anchor_node(&self) -> Option<Node> {
        js! (
            return @{self}.anchorNode;
        ).try_into().unwrap()
    }

    /// Returns the number of characters that the selection's anchor is offset within the
    /// [anchor_node](struct.Selection.html#method.anchor_node).
    ///
    /// This number is zero-based. If the selection begins with the first character in the
    /// [anchor_node](struct.Selection.html#method.anchor_node), 0 is returned.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)
    pub fn anchor_offset(&self) -> u32 {
        js! (
            return @{self}.anchorOffset;
        ).try_into().unwrap()
    }

    /// Returns the [Node](struct.Node.html) in which the selection ends.
    ///
    /// A user may make a selection from left to right (in document order) or right to left
    /// (reverse of document order). The focus is where the user ended the selection. This can be
    /// visualized by holding the Shift key and pressing the arrow keys on your keyboard to modify
    /// the current selection. The selection's focus moves, but the selection's anchor, the other
    /// end of the selection, does not move.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusNode)
    pub fn focus_node(&self) -> Option<Node> {
        js! (
            return @{self}.focusNode;
        ).try_into().unwrap()
    }

    /// Returns the number of characters that the selection's anchor is offset within the
    /// [focus_node](struct.Selection.html#method.focus_node).
    ///
    /// This number is zero-based. If the selection begins with the first character in the
    /// [focus_node](struct.Selection.html#method.focus_node), 0 is returned.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/focusOffset)
    pub fn focus_offset(&self) -> u32 {
        js! (
            return @{self}.focusOffset;
        ).try_into().unwrap()
    }

    /// Returns a boolean which indicates whether or not there is currently any text selected; That
    /// is to say that the selection's start and end points are at the same position in the
    /// content.
    ///
    /// Keep in mind that a collapsed selection may still have one (or more, in Gecko)
    /// [Range](struct.Range.html)s, so [range_count](struct.Selection.html#method.range_count) may
    /// not be zero. In that scenario, calling a [Selection](struct.Selection.html) object's
    /// [get_range_at](struct.Selection.html#method.get_range_at) method may return a
    /// [Range](struct.Range.html) object which is collapsed.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/isCollapsed)
    pub fn is_collapsed(&self) -> bool {
        js! (
            return @{self}.isCollapsed;
        ).try_into().unwrap()
    }

    /// Returns the number of ranges in the selection.
    ///
    /// Before the user has clicked a freshly loaded page, the
    /// [range_count](struct.Selection.html#method.range_count) is 0. After the user
    /// clicks on the page, [range_count](struct.Selection.html#method.range_count) even if no 
    /// selection is visible.
    ///
    /// A user can normally only select one range at a time, so the 
    /// [range_count](struct.Selection.html#method.range_count) will usually be 1.
    /// Scripting can be used to make the selection contain more than 1 range.
    ///
    /// Gecko browsers allow multiple selections across table cells.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/rangeCount)
    pub fn range_count(&self) -> u32 {
        js! (
            return @{self}.rangeCount;
        ).try_into().unwrap()
    }

    /// Returns the type of the current selection.
    ///
    /// Possible values are:
    ///
    /// * `None`: No selection has currently been made.
    /// * `Caret`: The selection is collapsed (i.e. the caret is placed on some text, but no
    /// range has been selected).
    /// * `Range`: A range has been selected.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/type)
    pub fn kind(&self) -> SelectionType {
        let selection: String = js! (
            return @{self}.type;
        ).try_into().unwrap();

        match selection.as_ref() {
            "None" => SelectionType::None,
            "Caret" => SelectionType::Caret,
            "Range" => SelectionType::Range,
            _ => panic!("Selection Type invalid!"),
        }
    }

    /// Returns a [Range](struct.Range.html) object representing one of the ranges currently
    /// selected.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)
    pub fn get_range_at(&self, index: u32) -> Result<Range, IndexSizeError> {
        js_try! (
            return @{self}.getRangeAt(@{index});
        ).unwrap()
    }

    /// Adds a [Range](struct.Range.html) to the [Selection](struct.Selection.html).
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)
    pub fn add_range(&self, range: &Range) {
        js! { @(no_return)
            @{self}.addRange(@{range});
        };
    }

    /// Removes a [Range](struct.Range.html) from the [Selection](struct.Selection.html).
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)
    pub fn remove_range(&self, range: &Range) -> Result<(), NotFoundError> {
        js_try! ( @(no_return)
            @{self}.removeRange(@{range});
        ).unwrap()
    }

    /// Removes all ranges from the [Selection](struct.Selection.html), leaving the
    /// [anchor_node](struct.Selection.html#method.anchor_node) and
    /// [focus_node](struct.Selection.html#method.focus_node) properties equal to
    /// `None` and leaving nothing selected.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)
    pub fn remove_all_ranges(&self) {
        js! { @(no_return)
            @{self}.removeAllRanges();
        };
    }

    /// Collapses the [Selection](struct.Selection.html) to a single point. The document is not
    /// modified. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    pub fn collapse<N: INode>(&self, node: &N) {
        js! { @(no_return)
            @{self}.collapse(@{node.as_ref()});
        }
    }

    /// Collapses the [Selection](struct.Selection.html) to a single point. The document is not
    /// modified. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    pub fn collapse_with_offset<N: INode>(&self, node: &N, offset: Option<u32>) -> Result<(), IndexSizeError> {
        js_try! ( @(no_return)
            @{self}.collapse(@{node.as_ref()}, @{offset});
        ).unwrap()
    }

    /// Collapses the [Selection](struct.Selection.html) to the start of the first range in the
    /// selection. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)
    pub fn collapse_to_start(&self) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.collapseToStart();
        ).unwrap()
    }

    /// Collapses the [Selection](struct.Selection.html) to the end of the last range in the
    /// selection. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)
    pub fn collapse_to_end(&self) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.collapseToEnd();
        ).unwrap()
    }

    /// Moves the focus of the selection to a specified point. The anchor of the selection does not
    /// move. The selection will be from the anchor node to the new focus regardless of direction.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)
    pub fn extend<N: INode>(&self, node: &N, offset: Option<u32>) -> Result<(), InvalidStateError> {
        js_try! ( @(no_return)
            @{self}.extend(@{node.as_ref()}, @{offset});
        ).unwrap()
    }

    /// Sets the selection to be a range including all or parts of the two specified
    /// [Node](struct.Node.html)s, and any content located between them.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)
    pub fn set_base_and_extent<N: INode, M: INode>(&self, anchor_node: &N, anchor_offset: u32, focus_node: &M, focus_offset: u32) -> Result<(), IndexSizeError> {
        js_try! ( @(no_return)
            @{self}.setBaseAndExtent(@{anchor_node.as_ref()}, @{anchor_offset}, @{focus_node.as_ref()}, @{focus_offset});
        ).unwrap()
    }

    /// Adds all the children of the specified [Node](struct.Node.html) to the selection. Previous
    /// selection is lost.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/selectAllChildren)
    pub fn select_all_children<N: INode>(&self, node: &N) {
        js! { @(no_return)
            @{self}.selectAllChildren(@{node.as_ref()});
        };
    }

    /// Deletes the actual text being represented by the [Selection](struct.Selection.html) from
    /// the document's DOM.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)
    pub fn delete_from_document(&self) {
        js! { @(no_return)
            @{self}.deleteFromDocument();
        };
    }

    /// Indicates if the entire node is part of the selection.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    pub fn contains_whole<N: INode>(&self, node: &N) -> bool {
        js! (
            return @{self}.containsNode(@{node.as_ref()}, false);
        ).try_into().unwrap()
    }

    /// Indicates if atleast some of the node is part of the selection.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    pub fn contains_part_of<N: INode>(&self, node: &N) -> bool {
        js! (
            return @{self}.containsNode(@{node.as_ref()}, true);
        ).try_into().unwrap()
    }
}

/// The Range interface represents a fragment of a document that can contain nodes and parts of
/// text nodes.
///
/// A range can be created using the [create_range()](struct.Document.html#method.create_range) method
/// of the Document object. Range objects can also be retrieved by using the
/// [get_range_at()](struct.Selection.html#method.get_range_at) method of the [Selection](struct.Selection.html)
/// object or the [caret_range_from_point()](struct.Document.html#method.caret_range_from_point) method of
/// the [Document](struct.Document.html] object.
// https://dom.spec.whatwg.org/#range
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "Range")]
pub struct Range(Reference);

impl Range {
    /// Returns a boolean indicating whether the range's start and end points are at the same
    /// position.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/collapsed)
    pub fn collapsed(&self) -> bool {
        js! (
            return @{self}.collapsed;
        ).try_into().unwrap()
    }

    /// Returns the deepest [Node](struct.Node.html) that contains the startContainer and
    /// endContainer nodes.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/commonAncestorContainer)
    pub fn common_ancestor_container(&self) -> Node {
        js! (
            return @{self}.commonAncestorContainer;
        ).try_into().unwrap()
    }

    /// Returns the [Node](struct.Node.html) within which the [Range](struct.Range.html) ends.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/endContainer)
    pub fn end_container(&self) -> Node {
        js! (
            return @{self}.endContainer;
        ).try_into().unwrap()
    }

    /// Returns a number representing where in the endContainer the [Range](struct.Range.html) ends.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/endOffset)
    pub fn end_offset(&self) -> u32 {
        js! (
            return @{self}.endOffset;
        ).try_into().unwrap()
    }

    /// Returns the [Node](struct.Node.html) within which the [Range](struct.Range.html) starts.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/startContainer)
    pub fn start_container(&self) -> Node {
        js! (
            return @{self}.startContainer;
        ).try_into().unwrap()
    }

    /// Returns a number representing where in the startContainer the [Range](struct.Range.html) starts.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/startOffset)
    pub fn start_offset(&self) -> u32 {
        js! (
            return @{self}.startOffset;
        ).try_into().unwrap()
    }
}

#[cfg(all(test, feature = "web_test"))]
mod tests {
    use super::*;
    use webapi::document::document;
    use webapi::window::window;

    fn div() -> Node {
        let node = js!(
            return document.createElement("div");
        ).try_into().unwrap();
        document().body().unwrap().append_child(&node);
        node
    }

    fn text(text: &str) -> Node {
        js!(
            return new Text(@{text});
        ).try_into().unwrap()
    }

    fn selection() -> Selection {
        window().get_selection().unwrap()
    }

    #[test]
    fn test_set_base_and_extent() {
        let parent = div();
        parent.append_child(&text("ab"));

        assert!(selection().set_base_and_extent(&parent, 0, &parent, 0).is_ok());
    }

    #[test]
    fn test_anchor() {
        let parent = div();
        parent.append_child(&text("ab"));
        assert!(selection().set_base_and_extent(&parent, 0, &parent, 0).is_ok());
        assert_eq!(selection().anchor_node().unwrap().as_ref(), parent.as_ref());
        assert_eq!(selection().anchor_offset(), 0);
    }

    #[test]
    fn test_focus() {
        let parent = div();
        parent.append_child(&text("ab"));
        assert!(selection().set_base_and_extent(&parent, 0, &parent, 0).is_ok());
        assert_eq!(selection().focus_node().unwrap().as_ref(), parent.as_ref());
        assert_eq!(selection().focus_offset(), 0);
    }

    #[test]
    fn test_is_collapsed() {
        let parent = div();
        parent.append_child(&text("ab"));
        assert!(selection().set_base_and_extent(&parent, 0, &parent, 0).is_ok());
        assert!(selection().is_collapsed());
    }

    #[test]
    fn test_contains_part_of() {
        let parent = div();
        parent.append_child(&text("ab"));
        assert!(selection().set_base_and_extent(&parent, 0, &parent, 0).is_ok());
        assert!(selection().contains_part_of(&parent));
    }

    #[test]
    fn test_contains_whole() {
        let parent = div();
        let text_node = text("ab");
        parent.append_child(&text_node);
        selection().select_all_children(&parent);
        assert!(selection().contains_whole(&text_node));
    }
}
