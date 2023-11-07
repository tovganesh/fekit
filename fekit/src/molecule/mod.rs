/** molecule module consists of basic structs: Atom, AtomGroup, Molecule */

mod molecule {
    /** Point - represents a point in space */
    #[allow(dead_code)]
    #[derive(Default, Clone, Copy)]
    pub struct Point {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    /** Atom is the primary struct used to define an AtomGroup or a Molecule */
    #[allow(dead_code)]
    #[derive(Default)]
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
                remark: self.remark.to_string()
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
                remark: self.atom_list[index].remark.to_string()
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

    #[allow(dead_code)]
    pub enum BondType {
        SINGLE,
        DOUBLE,
        TRIPLE,
        AROMATIC,
        WEAK,
    }

    #[allow(dead_code)]
    pub struct Bond {
        pub atom_a: Atom,
        pub atom_b: Atom,
        pub bond_type: BondType,
    }

    #[allow(dead_code)]
    pub struct Molecule {
        pub name: String,
        pub remark: String,

        atom_list: Vec<Atom>,
        bond_list: Vec<Bond>,
    }
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    use super::molecule::{self, AtomGroup};

    #[test]
    fn atom_init() {
        let atom = molecule::Atom {
            center: molecule::Point {
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

    #[test]
    fn atom_group_init() {
        let atom_group: AtomGroup =
            molecule::AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        assert_eq!(atom_group.name, "OH".to_string());
        assert_eq!(atom_group.remark, "Alcohol".to_string());
    }

    #[test]
    fn atom_group_fn() {
        let mut atom_group: AtomGroup =
            molecule::AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        atom_group.add_atom(molecule::Atom {
            center: molecule::Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        atom_group.add_atom(molecule::Atom {
            center: molecule::Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        });

        let atom_1 = atom_group.get_atom(0);
        assert!(atom_1.center.x == 0.0);
        assert!(atom_1.center.y == 0.0);
        assert!(atom_1.center.z == 0.0);
        assert!(atom_1.charge == 0.0);
        assert!(atom_1.symbol == "H".to_string());
        assert!(atom_1.remark == "Hydrogen Atom".to_string());

        let atom_2 = atom_group.get_atom(1);
        assert!(atom_2.center.x == 1.0);
        assert!(atom_2.center.y == 0.0);
        assert!(atom_2.center.z == 0.0);
        assert!(atom_2.charge == 0.0);
        assert!(atom_2.symbol == "O".to_string());
        assert!(atom_2.remark == "Oxygen Atom".to_string());
    }
}
