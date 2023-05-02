use std::env;

pub fn clear_env(keepers: &Vec<String>) {
    for (key,_) in env::vars() {
        let mut keep_it = false;
        for keeper in keepers {
            if &key == keeper {
                keep_it = true;
                continue;
            }
        }
        if ! keep_it {
            env::remove_var(key);
        }
    }
}


