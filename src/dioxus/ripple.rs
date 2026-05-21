//! Dioxus click-ripple hook + overlay.
//!
//! Mirrors leptos-daisyui-rs's `use_ripple` (class prefix `ld-` → `trs-`).
//! Bookkeeping lives in [`crate::core::ripple`]; the animation is the CSS
//! shipped with the motion system. Dismissal is `onanimationend`-driven,
//! so no timer is required.

use dioxus::prelude::*;

use crate::core::{RippleInstance, dismiss_instance, push_instance};

/// Handle returned by [`use_ripple`]. `Copy` (Dioxus signals are `Copy`).
#[derive(Clone, Copy)]
pub struct RippleHandle {
    instances: Signal<Vec<RippleInstance>>,
    next_id: Signal<u64>,
}

/// Create ripple bookkeeping for the calling component. Invoke
/// [`RippleHandle::trigger`] from `onclick` and render
/// [`RippleHandle::overlay`] inside a `trs-ripple-host` element.
pub fn use_ripple() -> RippleHandle {
    let instances = use_signal(Vec::<RippleInstance>::new);
    let next_id = use_signal(|| 0_u64);
    RippleHandle { instances, next_id }
}

impl RippleHandle {
    /// Spawn a ripple at the event's element-relative coordinates.
    pub fn trigger(&self, e: &Event<MouseData>) {
        let mut next_id = self.next_id;
        let mut instances = self.instances;
        let id = next_id();
        next_id.set(id.wrapping_add(1));
        let coords = e.element_coordinates();
        let inst = RippleInstance {
            id,
            x: coords.x as i32,
            y: coords.y as i32,
        };
        let mut v = instances();
        push_instance(&mut v, inst);
        instances.set(v);
    }

    /// Remove the ripple with `id` (called from `onanimationend`).
    pub fn dismiss(&self, id: u64) {
        let mut instances = self.instances;
        let mut v = instances();
        dismiss_instance(&mut v, id);
        instances.set(v);
    }

    /// Render the live ripple `<span>`s. Place inside the
    /// `trs-ripple-host` element that triggers the ripples.
    pub fn overlay(&self) -> Element {
        let handle = *self;
        let instances = handle.instances;
        rsx! {
            for r in instances().into_iter() {
                span {
                    key: "{r.id}",
                    class: "trs-ripple-element",
                    style: "left:{r.x}px;top:{r.y}px",
                    onanimationend: move |_| handle.dismiss(r.id),
                }
            }
        }
    }
}
