pub fn add_null_term(str: &[u8]) -> Vec<u8> {
    let mut str = Vec::from(str);
    str.push(b'\0');

    str
}

pub fn calculate_center_of_triangle(
    pos1: (f32, f32),
    pos2: (f32, f32),
    pos3: (f32, f32),
) -> (f32, f32) {
    let x_avg = (pos1.0 + pos2.0 + pos3.0) / 3.0;
    let y_avg = (pos1.1 + pos2.1 + pos3.1) / 3.0;

    (x_avg, y_avg)
}
pub fn calculate_distance(pos1: (f32, f32), pos2: (f32, f32)) -> f32 {
    ((pos2.0 - pos1.0).powi(2) + (pos2.1 - pos1.1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use core::f32;

    use super::*;

    #[test]
    fn test_calculate_distance() {
        assert_eq!(calculate_distance((0.0, 1.0), (0.0, 2.0)), 1.0);
        assert_eq!(
            calculate_distance((0.0, 1.0), (1.0, 2.0)),
            f32::consts::SQRT_2
        );
    }

    #[test]
    fn test_calculate_center_of_triangle() {
        assert_eq!(
            calculate_center_of_triangle((0.0, 0.0), (5.0, 0.0), (3.0, 5.0)),
            (2.666_666_7, 1.666_666_6)
        );
    }
}
