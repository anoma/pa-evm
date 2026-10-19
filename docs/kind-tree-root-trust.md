# The kind tree root is trusted for balance

The compliance circuit gets the kind point of a resource in one of two ways. Without a kind path, it computes the point by hash to curve from the logic ref and the label ref of the resource. With a kind path, it reads the point from a leaf of the kind tree and checks the leaf against the kind tree root that the transaction carries. The circuit takes a point from the tree as given. It requires the uncompressed SEC1 encoding, which rules out the identity, but it cannot tell a point made by hash to curve from a point whose discrete log is known.

The unit delta adds up the quantity of each resource times its kind point, plus `rcv` times the generator, and the delta proof shows knowledge of the discrete log of the sum. If the discrete log of a kind point is known, the prover can fold any quantity of that kind into that discrete log. A transaction proven against a kind tree with such a point can therefore create resources of its kind from nothing.

The protocol adapter accepts a transaction proven against the stored kind tree root, so the account that calls `setKindTreeRoot` is trusted for balance as much as the account that upgrades the implementation is. Both are the proxy owner today. A deployment that gives the two roles to different accounts has to trust the one that sets the root just as much.
