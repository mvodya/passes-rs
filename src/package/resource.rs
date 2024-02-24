use std::{
    io::{Read, Write},
    str::FromStr,
};

use regex::Regex;

/// Represents image file, saved in .pkpass package
#[derive(Debug)]
pub struct Resource {
    /// Type of image (represents file name)
    image_type: Type,

    /// File buffer
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

    /// Get resource data
    pub fn as_bytes(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    // Get resource file name
    pub fn filename(&self) -> String {
        self.image_type.to_string()
    }

    /// Get resource type
    pub fn get_type(&self) -> Type {
        self.image_type.clone()
    }
}

// Reading resource data
impl Write for Resource {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.buffer.flush()
    }
}

// Writing resource data
impl Read for Resource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        buf.as_mut().write(&self.buffer)
    }
}

/// Image size versions
#[derive(Debug, PartialEq, Clone)]
pub enum Version {
    Standard,
    Size2X,
    Size3X,
}

// To String
impl ToString for Version {
    fn to_string(&self) -> String {
        match self {
            Version::Standard => "".into(),
            Version::Size2X => "@2x".into(),
            Version::Size3X => "@3x".into(),
        }
    }
}

// From String
impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Version::Standard),
            "@2x" => Ok(Version::Size2X),
            "@3x" => Ok(Version::Size3X),
            _ => Err(()),
        }
    }
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
#[derive(Debug, PartialEq, Clone)]
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

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Background(v) => format!("background{}.png", v.to_string()),
            Type::Footer(v) => format!("footer{}.png", v.to_string()),
            Type::Icon(v) => format!("icon{}.png", v.to_string()),
            Type::Logo(v) => format!("logo{}.png", v.to_string()),
            Type::Strip(v) => format!("strip{}.png", v.to_string()),
            Type::Thumbnail(v) => format!("thumbnail{}.png", v.to_string()),
        }
    }
}

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Note: format field unused
        let re = Regex::new(r"(?P<type>\w+)(?P<version>@\dx)?\.(?P<format>png)").unwrap();
        let captures = re.captures(&s);

        // Extract captures
        if let Some(captures) = captures {
            // Extract version
            let version = if let Some(version) = captures.name("version") {
                version
                    .as_str()
                    .parse::<Version>()
                    .unwrap_or(Version::Standard)
            } else {
                Version::Standard
            };

            if let Ok(image_type_str) = captures["type"].parse::<String>() {
                // Match type & version
                match image_type_str.as_str() {
                    "background" => Ok(Type::Background(version)),
                    "footer" => Ok(Type::Footer(version)),
                    "icon" => Ok(Type::Icon(version)),
                    "logo" => Ok(Type::Logo(version)),
                    "strip" => Ok(Type::Strip(version)),
                    "thumbnail" => Ok(Type::Thumbnail(version)),
                    _ => Err(()),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
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

    #[test]
    fn check_type_string() {
        let t = Type::Footer(Version::Standard);
        assert_eq!("footer.png", t.to_string());

        let t = Type::Logo(Version::Size2X);
        assert_eq!("logo@2x.png", t.to_string());
    }

    #[test]
    fn check_type_from_string() {
        let t = "footer.png".parse::<Type>().unwrap();
        assert_eq!(Type::Footer(Version::Standard), t);

        let t = Type::from_str("logo@2x.png").unwrap();
        assert_eq!(Type::Logo(Version::Size2X), t);
    }
}
