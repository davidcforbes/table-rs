//! Leptos click-ripple hook + overlay.
//!
//! Port of leptos-daisyui-rs's `use_ripple` with the class prefix
//! `ld-` → `trs-`. The bookkeeping lives in [`crate::core::ripple`]; the
//! animation is the CSS shipped with the motion system. Dismissal is
//! driven by `animationend`, so no timer is needed.

use leptos::prelude::*;

use crate::core::{RippleInstance, dismiss_instance, push_instance};

/// Reactive handle returned by [`use_ripple`]. Cheap to copy.
#[derive(Clone, Copy)]
pub struct RippleHandle {
    /// Call from `on:click` to spawn a ripple at the event coordinates.
    pub trigger: Callback<web_sys::MouseEvent>,
    /// Read by [`RippleOverlay`] to render the live spans.
    pub instances: ReadSignal<Vec<RippleInstance>>,
    /// Removes the ripple with the given id (called on `animationend`).
    pub dismiss: Callback<u64>,
}

/// Create ripple bookkeeping. Invoke `trigger` from `on:click` and render
/// `instances` via [`RippleOverlay`] inside a `trs-ripple-host` element.
pub fn use_ripple() -> RippleHandle {
    let (instances, set_instances) = signal(Vec::<RippleInstance>::new());
    let next_id = RwSignal::new(0_u64);

    let trigger = Callback::new(move |ev: web_sys::MouseEvent| {
        let id = next_id.get_untracked();
        next_id.set(id.wrapping_add(1));
        let inst = RippleInstance {
            id,
            x: ev.offset_x(),
            y: ev.offset_y(),
        };
        set_instances.update(|v| push_instance(v, inst));
    });

    let dismiss = Callback::new(move |id: u64| {
        set_instances.update(|v| dismiss_instance(v, id));
    });

    RippleHandle {
        trigger,
        instances,
        dismiss,
    }
}

/// Renders the live ripple `<span>`s. Place inside a `trs-ripple-host`
/// element (the host supplies `position: relative; overflow: hidden`).
#[component]
pub fn RippleOverlay(handle: RippleHandle) -> impl IntoView {
    view! {
        <For
            each=move || handle.instances.get()
            key=|r| r.id
            children=move |r| {
                view! {
                    <span
                        class="trs-ripple-element"
                        style:left=format!("{}px", r.x)
                        style:top=format!("{}px", r.y)
                        on:animationend=move |_| handle.dismiss.run(r.id)
                    />
                }
            }
        />
    }
}
