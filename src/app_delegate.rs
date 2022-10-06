use crate::{AppData, APP_SIG};
use druid::{AppDelegate, DelegateCtx, Env, WindowId};
use preferences::Preferences;

pub struct Delegate;

impl Delegate {
    pub fn new() -> Self {
        Self
    }
}

impl AppDelegate<AppData> for Delegate {
    fn window_removed(
        &mut self,
        _id: WindowId,
        data: &mut AppData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        let save_result = data.spline.save(&APP_SIG, "saved_spline");
        if !save_result.is_ok() {
            println!("Error saving current spline: {:?}", save_result);
        }
    }
}
