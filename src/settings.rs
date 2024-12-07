/*---------------------------------------- PLUGIN SETTINGS ---------------------------------------*/

use nvim_oxi::Dictionary;

#[derive(Debug)]
struct SettingsTask {
    insert_mode_on_launch: bool,
}

#[derive(Debug)]
pub(crate) struct Settings {
    confirm_choice: bool,
    task: SettingsTask,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            confirm_choice: false,
            task: SettingsTask { insert_mode_on_launch: false },
        }
    }

    fn apply(&mut self, dict: Dictionary) {
        unsafe {
            self.confirm_choice = dict.get("confirm_choice").unwrap().as_boolean_unchecked();
            self.task.insert_mode_on_launch = dict
                .get("task")
                .cloned()
                .unwrap()
                .into_dict_unchecked()
                .get("insert_mode_on_launch")
                .unwrap()
                .as_boolean_unchecked();
        }
    }
}

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
        crate::STATE
            .borrow_mut()
            .settings
            .apply(Dictionary::from_iter(active_settings));
    }
}
