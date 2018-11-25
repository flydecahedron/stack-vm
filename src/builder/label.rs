/// The Label struct
///
/// Contains:
/// * a `String` name.
/// * a `Option` of `usize`
#[derive(Debug, Clone)]
pub struct Label {
    name: String,
    ip: Option<usize>,
}

impl Label {
    /// Create a new `Label` by name.
    pub fn new(name: &str) -> Label {
        Label {
            name: name.to_string(),
            ip: None,
        }
    }

    /// The name of this label.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The instruction pointer of this label.
    pub fn ip(&self) -> Option<usize> {
        self.ip
    }

    /// Set the instruction pointer for this label.
    pub fn set_ip(&mut self, ip: usize) {
        self.ip = Some(ip)
    }

    /// Has an instruction pointer been set on this label?
    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let label = Label::new("Marty");
        assert_eq!(label.name, "Marty");
        assert!(label.ip.is_none());
    }

    #[test]
    fn name() {
        let label = Label::new("Marty");
        assert_eq!(label.name(), "Marty");
    }

    #[test]
    fn set_ip() {
        let mut label = Label::new("Marty");
        assert!(label.ip.is_none());
        label.set_ip(123);
        assert_eq!(label.ip.unwrap(), 123);
    }

    #[test]
    fn has_ip() {
        let mut label = Label::new("Marty");
        assert!(!label.has_ip());
        label.set_ip(123);
        assert!(label.has_ip());
    }

    #[test]
    fn ip() {
        let mut label = Label::new("Marty");
        assert!(label.ip().is_none());
        label.set_ip(123);
        assert!(label.ip().is_some());
    }
}
