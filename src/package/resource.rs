use std::io::{Read, Write};

/// Represents image file, saved in .pkpass package
#[derive(Debug)]
pub struct Resource {
    /// Type of image
    image_type: Type,
    buffer: Vec<u8>,
}

impl Resource {
    /// Create new empty resource
    pub fn new(image_type: Type) -> Self {
        Self {
            image_type,
            buffer: vec![],
        }
    }

    // Get resource type
    pub fn get_type(self) -> Type {
        self.image_type
    }
}

// Reading resource data
impl Write for Resource {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer = buf.to_vec();
        Ok(self.buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// Writing resource data
impl Read for Resource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        buf.to_vec().write_all(&self.buffer)?;
        Ok(self.buffer.len())
    }
}

/// Image size versions
#[derive(Debug, PartialEq)]
pub enum Version {
    Standard,
    Size2X,
    Size3X,
}

/// Type of image.
///
/// * The background image (background.png) is displayed behind the entire front of the pass. The expected dimensions are 180 x 220 points. The image is cropped slightly on all sides and blurred. Depending on the image, you can often provide an image at a smaller size and let it be scaled up, because the blur effect hides details. This lets you reduce the file size without a noticeable difference in the pass.
/// * The footer image (footer.png) is displayed near the barcode. The allotted space is 286 x 15 points.
/// * The icon (icon.png) is displayed when a pass is shown on the lock screen and by apps such as Mail when showing a pass attached to an email. The icon should measure 29 x 29 points.
/// * The logo image (logo.png) is displayed in the top left corner of the pass, next to the logo text. The allotted space is 160 x 50 points; in most cases it should be narrower.
/// * The strip image (strip.png) is displayed behind the primary fields.
/// * On iPhone 6 and 6 Plus The allotted space is 375 x 98 points for event tickets, 375 x 144 points for gift cards and coupons, and 375 x 123 in all other cases.
/// * On prior hardware The allotted space is 320 x 84 points for event tickets, 320 x 110 points for other pass styles with a square barcode on devices with 3.5 inch screens, and 320 x 123 in all other cases.
/// * The thumbnail image (thumbnail.png) displayed next to the fields on the front of the pass. The allotted space is 90 x 90 points. The aspect ratio should be in the range of 2:3 to 3:2, otherwise the image is cropped.
#[derive(Debug, PartialEq)]
pub enum Type {
    /// The background image (background.png)
    Background(Version),
    /// The footer image (footer.png)
    Footer(Version),
    /// The icon (icon.png)
    Icon(Version),
    /// The logo image (logo.png)
    Logo(Version),
    /// The strip image (strip.png)
    Strip(Version),
    /// The thumbnail image (thumbnail.png)
    Thumbnail(Version),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_resource() {
        let mut data = [0u8; 2048];
        let mut resource = Resource::new(Type::Icon(Version::Standard));
        resource.write(&mut data).unwrap();

        println!("{}", resource.buffer.len());

        assert_eq!(resource.buffer.len(), 2048);
        assert_eq!(resource.get_type(), Type::Icon(Version::Standard));
    }
}
