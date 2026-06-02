use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ScrollViewer {
    pub key: Option<String>,
    pub modifiers: Modifiers,
    pub child: Box<Element>,
    pub horizontal_scroll_bar_visibility: ScrollBarVisibility,
    pub vertical_scroll_bar_visibility: ScrollBarVisibility,
    /// A changing token requesting scroll-to-bottom. When this value differs
    /// from the previous render, the backend scrolls to the bottom (deferred
    /// past layout). Use a monotonic counter that bumps when content is added.
    pub scroll_to_bottom: Option<f64>,
}
impl Default for ScrollViewer {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: Modifiers::default(),
            child: Box::new(Element::Empty),
            horizontal_scroll_bar_visibility: ScrollBarVisibility::Disabled,
            vertical_scroll_bar_visibility: ScrollBarVisibility::Auto,
            scroll_to_bottom: None,
        }
    }
}
impl ScrollViewer {
    pub fn new(child: impl Into<Element>) -> Self {
        Self {
            child: Box::new(child.into()),
            ..Default::default()
        }
    }
}

impl Widget for ScrollViewer {
    widget_header!(ControlKind::ScrollViewer);
    fn bindings(&self) -> PropBindings {
        let mut out = vec![
            Binding::Prop(
                Prop::HorizontalScrollBarVisibility,
                PropValue::ScrollVis(self.horizontal_scroll_bar_visibility),
            ),
            Binding::Prop(
                Prop::VerticalScrollBarVisibility,
                PropValue::ScrollVis(self.vertical_scroll_bar_visibility),
            ),
        ];
        if let Some(token) = self.scroll_to_bottom {
            out.push(Binding::Prop(Prop::ScrollToBottom, PropValue::F64(token)));
        }
        out
    }
    fn children(&self) -> Children<'_> {
        Children::PositionalSingle(&self.child)
    }
}

impl ScrollViewer {
    pub fn horizontal_scroll_bar_visibility(mut self, v: ScrollBarVisibility) -> Self {
        self.horizontal_scroll_bar_visibility = v;
        self
    }

    pub fn vertical_scroll_bar_visibility(mut self, v: ScrollBarVisibility) -> Self {
        self.vertical_scroll_bar_visibility = v;
        self
    }

    /// Request scroll-to-bottom whenever `token` changes between renders.
    pub fn scroll_to_bottom(mut self, token: f64) -> Self {
        self.scroll_to_bottom = Some(token);
        self
    }
}

pub fn scroll_viewer(child: impl Into<Element>) -> ScrollViewer {
    ScrollViewer::new(child)
}
