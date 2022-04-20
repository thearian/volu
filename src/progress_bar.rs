use indicatif::{ProgressStyle, MultiProgress, ProgressBar};
use std::path::PathBuf;

pub type ProgressOptional = Option<(ProgressStyle, MultiProgress)>;

pub fn init_progress_bar(should_not_init: bool) -> ProgressOptional {
    if should_not_init { return None }

    let progress_style = ProgressStyle::with_template(
        "{prefix:.bold.dim} {spinner}   {wide_msg}"
    ).unwrap()
        .tick_chars("⠁⠁⠂⠂⠄⠄⡀⡀⢀⢀⠠⠠⠐⠐⠈⠈");
    let progress_steam = MultiProgress::new();

    return Some((progress_style, progress_steam));
}

pub fn new_progress(progress: &ProgressOptional, size: &String) -> Option<ProgressBar> {
    match progress {
        None => None,
        Some((progress_style, progress_stream)) => {
            let pb = progress_stream.add(ProgressBar::new_spinner());
            pb.set_style(progress_style.clone());
            pb.set_prefix(
                format!("[{}]", size)
            );

            return Some(pb);
        }
    }
}

pub fn tick_progress(progress: &Option<ProgressBar>, child_path: &PathBuf) {
    match progress {
        None => {},
        Some(pb) => {
            pb.set_message(
                child_path.to_owned()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            );
            pb.inc(1);
        }
    }
}
