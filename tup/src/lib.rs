fn point(x:i32, y:i32, z:i32) -> (i32, i32, i32, i32) {
    (x,y,z,1)
}

fn vector(x:i32, y:i32, z:i32) -> (i32, i32, i32, i32) {
    (x,y,z,0)
}

fn add(first: (i32, i32, i32, i32), second: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let (x_first, y_first, z_first, type_first) = first;
    let (x_second, y_second, z_second, type_second) = second;

    (x_first + x_second, y_first + y_second, z_first + z_second, type_first + type_second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isPoint() {
        let result = point(1,2,3);
        assert_eq!(result.3, 1);

        let result = point(1,2,3);
        assert_ne!(result.3, 0);
    }

    #[test]
    fn isVector() {
        let result = vector(1,2,3);
        assert_eq!(result.3, 0);

        let result = vector(1,2,3);
        assert_ne!(result.3, 1);
    }

    #[test]
    fn adder() {
        let first = (1,2,3,0);
        let second  = (2,5,9,1);
        let result = add(first, second);
        assert_eq!(result, (3,7,12,1));
    }

    
}
