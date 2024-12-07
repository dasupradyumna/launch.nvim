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
    pub(crate) const fn new() -> Self {
        Self {
            confirm_choice: false,
            task: SettingsTask { insert_mode_on_launch: false },
        }
    }

    fn default() -> Dictionary {
        let mut default = Dictionary::new();

        default.insert("confirm_choice", false);
        default.insert("task", Dictionary::from_iter([("insert_mode_on_launch", false)]));

        default
    }

    pub(crate) fn apply(&mut self, user_settings: &Dictionary) {
        // TODO: implement argument validation

        let active = Dictionary::from_iter(Self::default().into_iter().map(|(k, default_v)| {
            (k.clone(), user_settings.get(&k).cloned().unwrap_or(default_v))
        }));

        unsafe {
            self.confirm_choice = active.get("confirm_choice").unwrap().as_boolean_unchecked();
            self.task.insert_mode_on_launch = active
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

// macro_rules! settings {
//     () => {};
// }
//
// settings! {
//     confirm_choice: bool,
//     task: {
//         insert_mode_on_launch: bool
//     }
// }
