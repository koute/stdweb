use webcore::value::Reference;
use webcore::try_from::TryInto;
use webapi::node::Node;
use private::TODO;

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
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/anchorOffset)
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
    pub fn selection_type(&self) -> String {
        js! (
            return @{self}.type;
        ).try_into().unwrap()
    }

    /// Returns a [Range](struct.Range.html) object representing one of the ranges currently
    /// selected.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/getRangeAt)
    pub fn get_range_at(&self, index: u32) -> Range {
        js! (
            return @{self}.getRangeAt(@{index});
        ).try_into().unwrap()
    }

    /// Adds a [Range](struct.Range.html) to the [Selection](struct.Selection.html).
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/addRange)
    pub fn add_range(&self, range: Range) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.addRange(@{range});
        };
        Ok(())
    }

    /// Removes a [Range](struct.Range.html) from the [Selection](struct.Selection.html).
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)
    pub fn remove_range(&self, range: Range) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.removeRange(@{range});
        };
        Ok(())
    }

    /// Removes all ranges from the [Selection](struct.Selection.html), leaving the
    /// [anchor_node](struct.Selection.html#method.anchor_node) and
    /// [focus_node](struct.Selection.html#method.focus_node) properties equal to
    /// `None` and leaving nothing selected.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/removeRange)
    pub fn remove_all_ranges(&self) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.removeAllRanges();
        };
        Ok(())
    }

    /// Collapses the [Selection](struct.Selection.html) to a single point. The document is not
    /// modified. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapse)
    pub fn collapse(&self, node: Node, offset: Option<u32>) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.collapse(@{node}, @{offset});
        };
        Ok(())
    }

    /// Collapses the [Selection](struct.Selection.html) to the start of the first range in the
    /// selection. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToStart)
    pub fn collapse_to_start(&self) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.collapseToStart();
        };
        Ok(())
    }

    /// Collapses the [Selection](struct.Selection.html) to the end of the last range in the
    /// selection. If the content is focused or editable, the caret will blink there.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/collapseToEnd)
    pub fn collapse_to_end(&self) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.collapseToEnd();
        };
        Ok(())
    }

    /// Moves the focus of the selection to a specified point. The anchor of the selection does not
    /// move. The selection will be from the anchor node to the new focus regardless of direction.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/extend)
    pub fn extend(&self, node: Node, offset: Option<u32>) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.extend(@{node}, @{offset});
        };
        Ok(())
    }

    /// Sets the selection to be a range including all or parts of the two specified
    /// [Node](struct.Node.html)s, and any content located between them.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/setBaseAndExtent)
    pub fn set_base_and_extent(&self, anchor_node: Node, anchor_offset: Option<u32>, focus_node: Node, focus_offset: Option<u32>) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.setBaseAndExtent(@{anchor_node}, @{anchor_offset}, @{focus_node}, @{focus_offset});
        };
        Ok(())
    }

    /// Adds all the children of the specified [Node](struct.Node.html) to the selection. Previous
    /// selection is lost.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/select_all_children)
    pub fn select_all_children(&self, node: Node) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.selectAllChildren(@{node});
        };
        Ok(())
    }

    /// Deletes the actual text being represented by the [Selection](struct.Selection.html) from
    /// the document's DOM.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/deleteFromDocument)
    pub fn delete_from_document(&self) -> Result<(), TODO> {
        js! { @(no_return)
            @{self}.deleteFromDocument();
        };
        Ok(())
    }

    /// Indicates if the node is part of the selection.
    ///
    /// [(Javascript
    /// docs)](https://developer.mozilla.org/en-US/docs/Web/API/Selection/containsNode)
    pub fn contains_node(&self, node: Node, allow_partial_containment: bool) -> bool {
        js! (
            return @{self}.containsNode(@{node}, @{allow_partial_containment});
        ).try_into().unwrap()
    }
}

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
    pub fn common_ancestor_container(&self) -> Option<Node> {
        js! (
            return @{self}.commonAncestorContainer;
        ).try_into().unwrap()
    }

    /// Returns the [Node](struct.Node.html) within which the [Range](struct.Range.html) ends.
    ///
    /// [(Javascript docs)](https://developer.mozilla.org/en-US/docs/Web/API/Range/endContainer)
    pub fn end_container(&self) -> Option<Node> {
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
    pub fn start_container(&self) -> Option<Node> {
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
