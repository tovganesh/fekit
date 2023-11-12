/** molecule module consists of basic structs: Atom, AtomGroup, Molecule */
use crate::atom::Atom;
use crate::bond::Bond;
use crate::bond::BondType;

#[allow(dead_code)]
pub struct Molecule {
    pub name: String,
    pub remark: String,

    atom_list: Vec<Atom>,
    bond_list: Vec<Bond>,
}

#[allow(dead_code)]
impl Molecule {
    pub fn new(name: String, remark: String) -> Molecule {
        Molecule {
            name: name,
            remark: remark,
            atom_list: Vec::new(),
            bond_list: Vec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: Atom) {
        self.atom_list.push(atom);
    }

    pub fn add_bond(&mut self, atom_1: Atom, atom_2: Atom, bond_type: BondType) {
        self.bond_list.push(Bond {
            atom_a: atom_1,
            atom_b: atom_2,
            bond_type: bond_type,
        });
    }
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    use crate::atom::Atom;
    use crate::atom::AtomGroup;
    use crate::point::Point;

    #[test]
    fn atom_init() {
        let atom = Atom {
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

    #[test]
    fn atom_group_init() {
        let atom_group: AtomGroup = AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        assert_eq!(atom_group.name, "OH".to_string());
        assert_eq!(atom_group.remark, "Alcohol".to_string());
    }

    #[test]
    fn atom_group_fn() {
        let mut atom_group: AtomGroup = AtomGroup::new("OH".to_string(), "Alcohol".to_string());

        atom_group.add_atom(Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        });
        atom_group.add_atom(Atom {
            center: Point {
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
