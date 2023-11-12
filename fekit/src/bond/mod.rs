use crate::atom::Atom;

/** BondType and Bond structs and methods */

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BondType {
    SINGLE,
    DOUBLE,
    TRIPLE,
    AROMATIC,
    WEAK,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct Bond {
    pub atom_a: Atom,
    pub atom_b: Atom,
    pub bond_type: BondType,
}

/** Unit tests for the above module */
#[cfg(test)]
mod tests {
    use crate::atom::Atom;
    use crate::point::Point;
    use crate::bond::Bond;
    use crate::bond::BondType;

    #[test]
    fn bond_init() {
        let atom_1 = Atom {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "H".to_string(),
            remark: "Hydrogen Atom".to_string(),
        };

        let atom_2 = Atom {
            center: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            charge: 0.0,
            symbol: "O".to_string(),
            remark: "Oxygen Atom".to_string(),
        };

        let bond = Bond {
            atom_a: atom_1,
            atom_b: atom_2,
            bond_type: BondType::SINGLE
        };

        assert_eq!(bond.atom_a.center.x, 0.0);
        assert_eq!(bond.atom_b.center.x, 1.0);
        assert_eq!(bond.atom_a.symbol, "H".to_string());
        assert_eq!(bond.atom_b.symbol, "O".to_string());
        assert_eq!(bond.bond_type, BondType::SINGLE);
    }
}