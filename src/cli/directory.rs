use super::CliImfconv;

pub struct AsDirectory;
impl CliImfconv for AsDirectory {
    fn exec(
        &self,
        src: &str,
        dest: &str,
        format: &imfconv::imfconv::ImageType,
        profile: &imfconv::imfconv::ColorProfile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
