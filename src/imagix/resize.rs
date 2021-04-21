pub fn get_image_files(src_folder: PathBuf) -> 
	Result<Vec<PathBuf>, ImagixError> {
		let entries = fs::read_dir(src_folder)
			.map_err(|e| ImageError::UserInputError("Invalid source folder".to_string()))?
			.map(|res| res.map(|e| e.path()))
			.collect::<Result<Vec<_>, io::Error>>()?
			.into_iter()
			.filter(|r| {
				r.extension() == Some("JPG".asref())
					||r.extension() == Some("jpg".asref())
					||r.extension() == Some("PNG".asref())
					||r.extension() == Some("jpg".asref())
			})
			.collect();
		Ok(entries)
}