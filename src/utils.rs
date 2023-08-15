use glob::glob;
use std::path::PathBuf;

const ACCEPTED_FILE_EXTS: [&'static str; 3] = ["webp", "jpg", "jpeg"];

#[derive(thiserror::Error, Debug)]
pub enum CyclerError {
    #[error("No files found while globbing")]
    PatternError(#[from] glob::PatternError),
    #[error("No valid image files")]
    NoValidImageFilesError,
    #[error("Directory not found")]
    DirectoryNotFound,
    #[error("Can't choose an image")]
    CantChooseAnImage,
    #[error("Can't convert PathBuf to Str")]
    CantConvertToStr,
}

pub fn get_valid_image_paths_from_provided_dir(
    backgrounds_path: &String,
) -> Result<Vec<PathBuf>, CyclerError> {
    let files_objects_in_provided_dir: glob::Paths = glob(&format!("{}/*", backgrounds_path))?;

    let valid_file_paths_in_provided_dir: Vec<PathBuf> = files_objects_in_provided_dir
        .filter_map(Result::ok)
        .collect();

    let valid_image_file_paths_in_provided_dir: Vec<PathBuf> = valid_file_paths_in_provided_dir
        .iter()
        .filter(|path| selected_file_is_valid_img(path))
        .map(PathBuf::to_owned)
        .collect();

    if valid_image_file_paths_in_provided_dir.len() == 0 {
        return Err(CyclerError::NoValidImageFilesError);
    }

    Ok(valid_image_file_paths_in_provided_dir)
}

pub fn selected_file_is_valid_img(selected_file: &PathBuf) -> bool {
    let os_ext = match selected_file.extension() {
        Some(os_ext) => os_ext,
        None => return false,
    };

    let ext = match os_ext.to_str() {
        Some(ext) => ext,
        None => return false,
    };

    ACCEPTED_FILE_EXTS.contains(&ext)
}
