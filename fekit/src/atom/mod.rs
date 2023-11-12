use crate::point::Point;

/** Atom is the primary struct used to define an AtomGroup or a Molecule */
#[allow(dead_code)]
#[derive(Default, PartialEq, Debug)]
pub struct Atom {
    pub center: Point,
    pub charge: f32,
    pub symbol: String,
    pub remark: String,
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        Atom {
            center: self.center,
            charge: self.charge,
            symbol: self.symbol.to_string(),
            remark: self.remark.to_string(),
        }
    }
}

/** AtomGroup is a collection of atoms with specific name */
#[allow(dead_code)]
pub struct AtomGroup {
    pub name: String,
    pub remark: String,

    atom_list: Vec<Atom>,
}

#[allow(dead_code)]
impl AtomGroup {
    pub fn new(name: String, remark: String) -> AtomGroup {
        AtomGroup {
            name: name,
            remark: remark,
            atom_list: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atom_list.push(atom)
    }

    pub fn get_atom(&mut self, index: usize) -> Atom {
        return Atom {
            center: self.atom_list[index].center,
            symbol: self.atom_list[index].symbol.to_string(),
            charge: self.atom_list[index].charge,
            remark: self.atom_list[index].remark.to_string(),
        };
    }

    pub fn remove_atom(&mut self, index: usize) -> Atom {
        self.atom_list.remove(index)
    }

    pub fn index_of(&mut self, atom: Atom) -> usize {
        return self
            .atom_list
            .iter()
            .position(|at| {
                at.center.x == atom.center.x
                    && at.center.y == atom.center.y
                    && at.center.z == atom.center.y
                    && at.symbol == atom.symbol
                    && at.charge == atom.charge
            })
            .unwrap();
    }
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    use crate::point::Point;

    #[test]
    fn atom_init() {
        let atom = super::Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "A Hydrogen Atom".to_string(),
        };

        assert_eq!(atom.center.x, 0.0);
        assert_eq!(atom.center.y, 0.0);
        assert_eq!(atom.center.z, 0.0);
        assert_eq!(atom.symbol, "H".to_string());
        assert_eq!(atom.remark, "A Hydrogen Atom".to_string());
    }
}