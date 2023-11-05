/** molecule module consists of basic structs: Atom, AtomGroup, Molecule */

mod molecule {
    /** Atom is the primary struct used to define an AtomGroup or a Molecule */
    #[allow(dead_code)]
    #[derive(Default)]
    pub struct Atom {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub charge: f32,
        pub symbol: String,
        pub remark: String,
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
            x: 0.0,
            y: 0.0,
            z: 0.0,
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "A Hydrogen Atom".to_string(),
        };

        assert_eq!(atom.x, 0.0);
        assert_eq!(atom.y, 0.0);
        assert_eq!(atom.z, 0.0);
        assert_eq!(atom.symbol, "H".to_string());
        assert_eq!(atom.remark, "A Hydrogen Atom".to_string());
    }

    #[test]
    fn atom_group_init() {
        let mut atom_group: AtomGroup =
            molecule::AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        atom_group.add_atom(molecule::Atom {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        atom_group.add_atom(molecule::Atom {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        });

        assert_eq!(atom_group.name, "OH".to_string());
        assert_eq!(atom_group.remark, "Alcohol".to_string());
    }
}
