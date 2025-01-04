/*------------------------------------ RUNTIME CONFIGURATIONS ------------------------------------*/

mod task;

pub(crate) use task::*;

use std::path::PathBuf;
use std::sync::LazyLock;

pub(crate) fn data_dir() -> &'static PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let ret: String = nvim_oxi::api::call_function::<_, String>("stdpath", ("data",)).unwrap();
        let mut ret = PathBuf::from(ret);
        ret.push("launch_nvim_rust");

        ret
    });

    &DATA_DIR
}
