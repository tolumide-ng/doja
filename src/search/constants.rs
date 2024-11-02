pub(crate) trait NodeType {
    /// Principal variatioin node
    const PV: bool;
    /// Root node of the search tree
    const ROOT: bool;
    type Next: NodeType;
}

pub(crate) struct Root;
impl NodeType for Root {
    const PV: bool = false;
    const ROOT: bool = true;
    type Next = Pv;
}

pub(crate) struct Pv;
impl NodeType for Pv {
    const PV: bool = true;
    const ROOT: bool = false;
    type Next = Self;
}

pub(crate) struct NotPv;
impl NodeType for NotPv {
    const PV: bool = false;
    const ROOT: bool = false;
    type Next = Self;
}

pub(crate) struct CheckForced;
impl NodeType for CheckForced {
    const PV: bool = false;
    const ROOT: bool = true;
    type Next = NotPv;
}