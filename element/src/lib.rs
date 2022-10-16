
#[derive(Debug)]
//#[derive(PartialEq)]
struct Element(i32, i32, i32, i32);

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3 {
            return true
        } else {
            return false
        }
    }
}

/*fn point(x:i32, y:i32, z:i32) -> Element {
   Element(x, y, z, 1)
}

fn vector(x:i32, y:i32, z:i32) -> Element {
    Element(x,y,z,0)
}*/

fn add_them(first: Element, second: Element) -> Element {

    Element(first.0 + second.0, first.1 + second.1, first.2 + second.2, first.3 + second.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_point() {
        let result = Element(1,2,3,1);
        assert_eq!(result.3, 1);

        let result = Element(1,2,3,1);
        assert_ne!(result.3, 0);
    }

    #[test]
    fn is_vector() {
        let result = Element(1,2,3,0);
        assert_eq!(result.3, 0);

        let result = Element(1,2,3,0);
        assert_ne!(result.3, 1);
    }

    #[test]
    fn add_elements() {
        let first = Element(1,2,3,0);
        let second  = Element(2,5,9,1);
        let result = add_them(first, second);
        assert_eq!(result, Element(3,7,12,1));
    }

    
}
