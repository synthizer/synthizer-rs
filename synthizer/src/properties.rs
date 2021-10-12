use crate::internal_prelude::*;

/// Representation of a property backed by an i32.
pub struct IntProperty<'a> {
    handle: syz_Handle,
    property: i32,
    _pd: PhantomData<&'a ()>,
}

impl<'a> IntProperty<'a> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> IntProperty {
        IntProperty {
            handle: handle.to_syz_handle(),
            property,
            _pd: Default::default(),
        }
    }

    pub fn get(&self) -> Result<i32> {
        let mut out = 0;
        check_error(unsafe { syz_getI(&mut out as *mut i32, self.handle, self.property) })?;
        return Ok(out);
    }

    pub fn set(&self, value: i32) -> Result<()> {
        check_error(unsafe { syz_setI(self.handle, self.property, value) })
    }
}

/// A property backed by a Synthizer enum.
pub struct EnumProperty<'a, T> {
    iprop: IntProperty<'a>,
    _pd: std::marker::PhantomData<&'a T>,
}

impl<'a, T: I32TransmutableEnum> EnumProperty<'a, T> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> EnumProperty<T> {
        EnumProperty {
            iprop: IntProperty::new(handle, property),
            _pd: Default::default(),
        }
    }

    pub fn get(&self) -> Result<T> {
        let out = self.iprop.get()?;
        Ok(unsafe { T::from_i32(out) })
    }

    pub fn set(&self, value: T) -> Result<()> {
        self.iprop.set(value.as_i32())
    }
}

pub struct BoolProperty<'a> {
    iprop: IntProperty<'a>,
}

impl<'a> BoolProperty<'a> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> BoolProperty {
        BoolProperty {
            iprop: IntProperty::new(handle, property),
        }
    }

    pub fn get(&self) -> Result<bool> {
        Ok(self.iprop.get()? != 0)
    }

    pub fn set(&self, value: bool) -> Result<()> {
        self.iprop.set(value as i32)
    }
}

pub struct DoubleProperty<'a> {
    handle: syz_Handle,
    property: i32,
    _pd: PhantomData<&'a ()>,
}

impl<'a> DoubleProperty<'a> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> DoubleProperty {
        DoubleProperty {
            handle: handle.to_syz_handle(),
            property,
            _pd: Default::default(),
        }
    }

    pub fn get(&self) -> Result<f64> {
        let mut out = 0.0;
        check_error(unsafe { syz_getD(&mut out as *mut f64, self.handle, self.property) })?;
        Ok(out)
    }

    pub fn set(&self, value: f64) -> Result<()> {
        check_error(unsafe { syz_setD(self.handle, self.property, value) })
    }
}

pub struct Double3Property<'a> {
    handle: syz_Handle,
    property: i32,
    _pd: PhantomData<&'a ()>,
}

impl<'a> Double3Property<'a> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> Double3Property {
        Double3Property {
            handle: handle.to_syz_handle(),
            property,
            _pd: Default::default(),
        }
    }

    pub fn get(&self) -> Result<(f64, f64, f64)> {
        let mut o1 = 0.0;
        let mut o2 = 0.0;
        let mut o3 = 0.0;
        check_error(unsafe {
            syz_getD3(
                &mut o1 as *mut f64,
                &mut o2 as *mut f64,
                &mut o3 as *mut f64,
                self.handle,
                self.property,
            )
        })?;
        Ok((o1, o2, o3))
    }

    pub fn set(&self, values: (f64, f64, f64)) -> Result<()> {
        check_error(unsafe { syz_setD3(self.handle, self.property, values.0, values.1, values.2) })
    }
}

pub struct Double6Property<'a> {
    handle: syz_Handle,
    property: i32,
    _pd: PhantomData<&'a ()>,
}

impl<'a> Double6Property<'a> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> Double6Property {
        Double6Property {
            handle: handle.to_syz_handle(),
            property,
            _pd: Default::default(),
        }
    }

    pub fn get(&self) -> Result<(f64, f64, f64, f64, f64, f64)> {
        let mut o1 = 0.0;
        let mut o2 = 0.0;
        let mut o3 = 0.0;
        let mut o4 = 0.0;
        let mut o5 = 0.0;
        let mut o6 = 0.0;
        check_error(unsafe {
            syz_getD6(
                &mut o1 as *mut f64,
                &mut o2 as *mut f64,
                &mut o3 as *mut f64,
                &mut o4 as *mut f64,
                &mut o5 as *mut f64,
                &mut o6 as *mut f64,
                self.handle,
                self.property,
            )
        })?;
        Ok((o1, o2, o3, o4, o5, o6))
    }
}

pub struct ObjectProperty<'a> {
    handle: syz_Handle,
    property: i32,
    _pd: PhantomData<&'a ()>,
}

impl<'a> ObjectProperty<'a> {
    pub(crate) fn new(handle: &impl ToSyzHandle, property: i32) -> ObjectProperty {
        ObjectProperty {
            handle: handle.to_syz_handle(),
            property,
            _pd: Default::default(),
        }
    }

    pub fn set(&self, handle: &impl ToSyzHandle) -> Result<()> {
        check_error(unsafe { syz_setO(self.handle, self.property, handle.to_syz_handle()) })
    }
}
