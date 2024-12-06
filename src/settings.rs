/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/

use nvim_oxi::Dictionary;

pub(crate) static mut ACTIVE_SETTINGS: Option<Dictionary> = None;

fn default_settings() -> Dictionary {
    let mut default = Dictionary::new();

    default.insert("confirm_choice", false);
    default.insert("task", Dictionary::from_iter([("insert_mode_on_launch", false)]));

    default
}

pub(crate) fn apply(user_settings: &Dictionary) {
    // TODO: implement argument validation

    let active_settings = default_settings()
        .into_iter()
        .map(|(k, default_v)| (k.clone(), user_settings.get(&k).cloned().unwrap_or(default_v)));

    unsafe {
        ACTIVE_SETTINGS = Some(Dictionary::from_iter(active_settings));
    }
}
