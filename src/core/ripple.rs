//! Framework-agnostic ripple bookkeeping.
//!
//! The visible animation is pure CSS (`.trs-ripple-host`,
//! `.trs-ripple-element`, `@keyframes trs-ripple` — shipped with the
//! motion system). Each backend owns the reactive storage of active
//! instances in its idiomatic way; this module only provides the shared
//! data type and the two list mutations, so the spawn/dismiss semantics
//! are identical everywhere.

/// One active ripple. Coordinates are offsets within the host element so
/// the rendered `<span>` lands directly under the click.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RippleInstance {
    /// Monotonic id — used as the list key and the dismiss handle.
    pub id: u64,
    /// `MouseEvent::offset_x` in CSS pixels.
    pub x: i32,
    /// `MouseEvent::offset_y` in CSS pixels.
    pub y: i32,
}

/// Append a new ripple instance.
pub fn push_instance(v: &mut Vec<RippleInstance>, inst: RippleInstance) {
    v.push(inst);
}

/// Remove the ripple with `id` (no-op if absent). Called on `animationend`.
pub fn dismiss_instance(v: &mut Vec<RippleInstance>, id: u64) {
    v.retain(|r| r.id != id);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make(id: u64, x: i32, y: i32) -> RippleInstance {
        RippleInstance { id, x, y }
    }

    #[test]
    fn push_instance_appends_in_order() {
        let mut v = Vec::new();
        push_instance(&mut v, make(0, 10, 20));
        push_instance(&mut v, make(1, 30, 40));
        assert_eq!(v, vec![make(0, 10, 20), make(1, 30, 40)]);
    }

    #[test]
    fn dismiss_removes_by_id_only() {
        let mut v = vec![make(0, 0, 0), make(1, 1, 1), make(2, 2, 2)];
        dismiss_instance(&mut v, 1);
        assert_eq!(v, vec![make(0, 0, 0), make(2, 2, 2)]);
    }

    #[test]
    fn dismiss_unknown_id_is_noop() {
        let mut v = vec![make(0, 0, 0), make(1, 1, 1)];
        dismiss_instance(&mut v, 999);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn dismiss_then_repush_works() {
        let mut v = vec![make(0, 0, 0)];
        dismiss_instance(&mut v, 0);
        push_instance(&mut v, make(1, 5, 5));
        assert_eq!(v, vec![make(1, 5, 5)]);
    }

    #[test]
    fn instance_is_copy_and_structurally_equal() {
        let a = make(7, 1, 2);
        let b = a;
        assert_eq!(a, b);
        assert_ne!(make(0, 1, 2), make(1, 1, 2));
    }
}
