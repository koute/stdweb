use webcore::value::Reference;
use webcore::try_from::TryInto;
use webcore::serialization::JsSerialize;
use private::TODO;

/// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
// https://html.spec.whatwg.org/#history-3
#[derive(Clone, Debug, PartialEq, Eq, ReferenceType)]
#[reference(instance_of = "History")]
pub struct History(Reference);

impl History {
    /// Adds a new entry to history.
    ///
    /// pushState() takes three parameters: a state object, a title (which is currently ignored),
    /// and (optionally) a URL. Let's examine each of these three parameters in more detail:
    ///
    /// - state object — The state object is a JavaScript object which is associated with the new
    /// history entry created by pushState(). Whenever the user navigates to the new state, a
    /// popstate event is fired, and the state property of the event contains a copy of the history
    /// entry's state object.
    ///
    /// - title — Firefox currently ignores this parameter, although it may use it in the future.
    /// Passing the empty string here should be safe against future changes to the method.
    /// Alternatively, you could pass a short title for the state to which you're moving.
    ///
    /// - URL — The new history entry's URL is given by this parameter. Note that the browser won't
    /// attempt to load this URL after a call to pushState(), but it might attempt to load the URL
    /// later, for instance after the user restarts the browser. The new URL does not need to be
    /// absolute; if it's relative, it's resolved relative to the current URL. The new URL must be
    /// of the same origin as the current URL; otherwise, pushState() will throw an exception.
    /// This parameter is optional; if it isn't specified, it's set to the document's current URL.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History_API#The_pushState%28%29_method)
    // https://html.spec.whatwg.org/#the-history-interface:dom-history-pushstate
    pub fn push_state<T: JsSerialize>(&self, state: T, title: &str, url: Option<&str>) {
        js!{ @(no_return)
            @{self}.pushState(@{state}, @{title}, @{url});
        };
    }

    /// Operates exactly like history.push_state() except that replace_state() modifies the current
    /// history entry instead of creating a new one. Note that this doesn't prevent the creation of
    /// a new entry in the global browser history.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History_API#The_replaceState%28%29_method)
    // https://html.spec.whatwg.org/#the-history-interface:dom-history-replacestate
    pub fn replace_state<T: JsSerialize>(&self, state: T, title: &str, url: Option<&str>) -> Result< (), TODO > {
        js!{ @(no_return)
            @{self}.replaceState(@{state}, @{title}, @{url});
        };
        Ok(())
    }

    /// You can use the go() method to load a specific page from session history, identified by its
    /// relative position to the current page (with the current page being, of course, relative
    /// index 0).
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History_API#Traveling_through_history)
    // https://html.spec.whatwg.org/#the-history-interface:dom-history-go
    pub fn go(&self, offset: i32) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.go(@{offset});
        };
        Ok(())
    }

    /// Move one step backward through history.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History_API#Traveling_through_history)
    // https://html.spec.whatwg.org/#the-history-interface:dom-history-back
    pub fn back(&self) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.back();
        };
        Ok(())
    }

    /// Move one step forward through history.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History_API#Traveling_through_history)
    // https://html.spec.whatwg.org/#the-history-interface:dom-history-forward
    pub fn forward(&self) -> Result< (), TODO > {
        js! { @(no_return)
            @{self}.forward();
        };
        Ok(())
    }

    /// Returns the current number of history entries.
    ///
    /// [(JavaScript docs)](https://developer.mozilla.org/en-US/docs/Web/API/History)
    // https://html.spec.whatwg.org/#the-history-interface:dom-history-length
    pub fn len(&self) -> u32 {
        js!(
            return @{self}.length;
        ).try_into().unwrap()
    }
}
