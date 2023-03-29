#[cfg(test)]
mod tests {
    use crate::db::tiles::blank_tile;
    use crate::image::hsv_to_rgb;

    #[test]
    fn test_hsv2rgb() {
        let (r, g, b) = hsv_to_rgb(120.0, 31.4, 1.0);
        assert_eq!(r, 175);
        assert_eq!(g, 255);
        assert_eq!(b, 175);
    }

    #[test]
    fn test_hsv2rgb2() {
        let (r, g, b) = hsv_to_rgb(0.0, 1.0, 1.0);
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }
}