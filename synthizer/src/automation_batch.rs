use crate::internal_prelude::*;

/// Binds the Synthizer automation functionality.
///
/// This builds up a list of commands internally, and applies them to the batch
/// only when `flush` or ` execute` is called.  Applying them one by one to the
/// underlying Synthizer-side batch is otherwise very slow.
pub struct AutomationBatch {
    handle: syz_Handle,
    // We build up the commands as we go, and apply them to the batch all at
    // once at the end.  This improves performance significantly.
    commands: Vec<syz_AutomationCommand>,
}

impl AutomationBatch {
    pub fn new(ctx: &Context) -> Result<AutomationBatch> {
        let mut handle = 0;
        check_error(unsafe {
            syz_createAutomationBatch(
                &mut handle as *mut syz_Handle,
                ctx.to_syz_handle(),
                null_mut(),
                None,
            )
        })?;
        Ok(AutomationBatch {
            handle,
            commands: Default::default(),
        })
    }

    pub fn set_double(
        &mut self,
        target: &impl ToSyzHandle,
        property: DoubleProperty,
        time: f64,
        value: f64,
        interpolation_type: InterpolationType,
    ) -> Result<&mut Self> {
        let pcmd = syz_AutomationAppendPropertyCommand {
            point: syz_AutomationPoint {
                values: [value, 0.0, 0.0, 0.0, 0.0, 0.0],
                flags: 0,
                interpolation_type: interpolation_type as i32,
            },
            property: property.property,
        };

        let params = syz_AutomationCommandParams {
            append_to_property: pcmd,
        };

        let cmd = syz_AutomationCommand {
            time,
            flags: 0,
            target: target.to_syz_handle(),
            params,
            type_: SYZ_AUTOMATION_COMMAND_APPEND_PROPERTY as i32,
        };

        self.commands.push(cmd);
        Ok(self)
    }

    pub fn set_double3(
        &mut self,
        target: &impl ToSyzHandle,
        property: DoubleProperty,
        time: f64,
        value: (f64, f64, f64),
        interpolation_type: InterpolationType,
    ) -> Result<&mut Self> {
        let pcmd = syz_AutomationAppendPropertyCommand {
            point: syz_AutomationPoint {
                values: [value.0, value.1, value.2, 0.0, 0.0, 0.0],
                flags: 0,
                interpolation_type: interpolation_type as i32,
            },
            property: property.property,
        };

        let params = syz_AutomationCommandParams {
            append_to_property: pcmd,
        };

        let cmd = syz_AutomationCommand {
            time,
            flags: 0,
            target: target.to_syz_handle(),
            params,
            type_: SYZ_AUTOMATION_COMMAND_APPEND_PROPERTY as i32,
        };

        self.commands.push(cmd);
        Ok(self)
    }

    pub fn set_double6(
        &mut self,
        target: &impl ToSyzHandle,
        property: DoubleProperty,
        time: f64,
        value: (f64, f64, f64, f64, f64, f64),
        interpolation_type: InterpolationType,
    ) -> Result<&mut Self> {
        let pcmd = syz_AutomationAppendPropertyCommand {
            point: syz_AutomationPoint {
                values: [value.0, value.1, value.2, value.3, value.4, value.5],
                flags: 0,
                interpolation_type: interpolation_type as i32,
            },
            property: property.property,
        };

        let params = syz_AutomationCommandParams {
            append_to_property: pcmd,
        };

        let cmd = syz_AutomationCommand {
            time,
            flags: 0,
            target: target.to_syz_handle(),
            params,
            type_: SYZ_AUTOMATION_COMMAND_APPEND_PROPERTY as i32,
        };

        self.commands.push(cmd);
        Ok(self)
    }

    pub fn clear_all_properties(&mut self, target: &impl ToSyzHandle) -> Result<&mut Self> {
        let cmd = syz_AutomationCommand {
            target: target.to_syz_handle(),
            type_: SYZ_AUTOMATION_COMMAND_CLEAR_ALL_PROPERTIES as i32,
            ..Default::default()
        };

        self.commands.push(cmd);
        Ok(self)
    }

    pub fn clear_property(
        &mut self,
        target: &impl ToSyzHandle,
        property: impl SyzProperty,
    ) -> Result<&mut Self> {
        let pcmd = syz_AutomationClearPropertyCommand {
            property: property.as_i32(),
        };
        let params = syz_AutomationCommandParams {
            clear_property: pcmd,
        };
        let cmd = syz_AutomationCommand {
            type_: SYZ_AUTOMATION_COMMAND_CLEAR_PROPERTY as i32,
            target: target.to_syz_handle(),
            params,
            ..Default::default()
        };
        self.commands.push(cmd);
        Ok(self)
    }

    pub fn send_user_event(
        &mut self,
        target: &impl ToSyzHandle,
        time: f64,
        param: std::os::raw::c_ulonglong,
    ) -> Result<&mut Self> {
        let pcmd = syz_AutomationSendUserEventCommand { param };
        let params = syz_AutomationCommandParams {
            send_user_event: pcmd,
        };
        let cmd = syz_AutomationCommand {
            type_: SYZ_AUTOMATION_COMMAND_SEND_USER_EVENT as i32,
            target: target.to_syz_handle(),
            time,
            params,
            ..Default::default()
        };
        self.commands.push(cmd);
        Ok(self)
    }

    pub fn clear_user_events(&mut self, target: &impl ToSyzHandle) -> Result<&mut Self> {
        let cmd = syz_AutomationCommand {
            type_: SYZ_AUTOMATION_COMMAND_CLEAR_EVENTS as i32,
            target: target.to_syz_handle(),
            ..Default::default()
        };
        self.commands.push(cmd);
        Ok(self)
    }

    /// Flush all pending commands to the batch.
    pub fn flush(&mut self) -> Result<&mut Self> {
        check_error(unsafe {
            syz_automationBatchAddCommands(
                self.handle,
                self.commands.len() as u64,
                self.commands.as_mut_ptr(),
            )
        })?;
        self.commands.clear();
        Ok(self)
    }

    pub fn execute(mut self) -> Result<()> {
        self.flush()?;
        check_error(unsafe { syz_automationBatchExecute(self.handle) })?;
        Ok(())
    }
}

impl Drop for AutomationBatch {
    fn drop(&mut self) {
        unsafe {
            syz_handleDecRef(self.handle);
        }
    }
}
