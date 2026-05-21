//! Yew click-ripple hook + overlay helper.
//!
//! Mirrors leptos-daisyui-rs's `use_ripple` (class prefix `ld-` → `trs-`).
//! Bookkeeping lives in [`crate::core::ripple`]; the animation is the CSS
//! shipped with the motion system. Dismissal is `animationend`-driven, so
//! no timer (and thus no `gloo-timers`) is required.

use std::cell::RefCell;
use std::rc::Rc;

use web_sys::MouseEvent;
use yew::prelude::*;

use crate::core::{RippleInstance, dismiss_instance, push_instance};

/// Handle returned by [`use_ripple`]. Clone-cheap (shared state).
#[derive(Clone)]
pub struct RippleHandle {
    instances: UseStateHandle<Vec<RippleInstance>>,
    next_id: Rc<RefCell<u64>>,
}

/// Create ripple bookkeeping for the calling component. Invoke
/// [`RippleHandle::trigger`] from `onclick` and drop
/// [`RippleHandle::overlay`] inside a `trs-ripple-host` element.
#[hook]
pub fn use_ripple() -> RippleHandle {
    let instances = use_state(Vec::<RippleInstance>::new);
    let next_id = use_mut_ref(|| 0_u64);
    RippleHandle { instances, next_id }
}

impl RippleHandle {
    /// Spawn a ripple at the event's offset coordinates.
    pub fn trigger(&self, e: &MouseEvent) {
        let id = {
            let mut n = self.next_id.borrow_mut();
            let id = *n;
            *n = id.wrapping_add(1);
            id
        };
        let inst = RippleInstance {
            id,
            x: e.offset_x(),
            y: e.offset_y(),
        };
        let mut v = (*self.instances).clone();
        push_instance(&mut v, inst);
        self.instances.set(v);
    }

    /// Remove the ripple with `id` (called from `onanimationend`).
    pub fn dismiss(&self, id: u64) {
        let mut v = (*self.instances).clone();
        dismiss_instance(&mut v, id);
        self.instances.set(v);
    }

    /// Render the live ripple `<span>`s. Place inside the
    /// `trs-ripple-host` element that triggers the ripples.
    pub fn overlay(&self) -> Html {
        html! {
            { for (*self.instances).iter().map(|r| {
                let id = r.id;
                let handle = self.clone();
                let onanimationend = Callback::from(move |_| handle.dismiss(id));
                html! {
                    <span
                        class="trs-ripple-element"
                        style={format!("left:{}px;top:{}px", r.x, r.y)}
                        {onanimationend}
                    />
                }
            }) }
        }
    }
}
