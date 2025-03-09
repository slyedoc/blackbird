
use leptos::{prelude::*, svg};

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata_core::Icon>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] width: MaybeProp<String>,
    #[prop(into, optional)] height: MaybeProp<String>,
) -> impl IntoView {
    move || {
        let icon = icon.get();
        svg::svg()
            .style(match (style.get(), icon.style) {
                (Some(a), Some(b)) => Some(format!("{b} {a}")),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b.to_string()),
                _ => None,
            })
            .attr("class", class.get().unwrap_or_default())
            .attr("x", icon.x)
            .attr("y", icon.y)
            .attr("width", width.get().unwrap_or_else(|| "1em".to_string()))
            .attr("height", height.get().unwrap_or_else(|| "1em".to_string()))
            .attr("viewBox", icon.view_box)
            .attr("stroke-linecap", icon.stroke_linecap)
            .attr("stroke-linejoin", icon.stroke_linejoin)
            .attr("stroke-width", icon.stroke_width)
            .attr("stroke-linecap", icon.stroke_linecap)
            .attr("stroke", icon.stroke)
            .attr("fill", icon.fill.unwrap_or("currentColor"))
            .attr("role", "graphics-symbol")
            .inner_html(icon.data)
    }
}
