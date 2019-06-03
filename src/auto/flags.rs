// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use vte_sys;

bitflags! {
    pub struct PtyFlags: u32 {
        const NO_LASTLOG = 1;
        const NO_UTMP = 2;
        const NO_WTMP = 4;
        const NO_HELPER = 8;
        const NO_FALLBACK = 16;
        const DEFAULT = 0;
    }
}

#[doc(hidden)]
impl ToGlib for PtyFlags {
    type GlibType = vte_sys::VtePtyFlags;

    fn to_glib(&self) -> vte_sys::VtePtyFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<vte_sys::VtePtyFlags> for PtyFlags {
    fn from_glib(value: vte_sys::VtePtyFlags) -> PtyFlags {
        skip_assert_initialized!();
        PtyFlags::from_bits_truncate(value)
    }
}

impl StaticType for PtyFlags {
    fn static_type() -> Type {
        unsafe { from_glib(vte_sys::vte_pty_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PtyFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PtyFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for PtyFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

