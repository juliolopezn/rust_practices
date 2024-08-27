use rendezvous_hash::{NodeHasher, RendezvousNodes};
use std::hash::{Hash, Hasher};

pub struct Mummur3NodeHasher {
    seed: u32,
}

impl Mummur3NodeHasher {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}

impl<T: Hash> NodeHasher<T> for Mummur3NodeHasher {
    fn hash<H: Hash>(&self, node: &T, item: &H) -> u64 {
        let mut hasher = mur3::Hasher128::with_seed(self.seed);
        node.hash(&mut hasher);
        item.hash(&mut hasher);
        hasher.finish()
    }
}

fn main() { }

#[test]
fn test() {
    let hasher = Mummur3NodeHasher::new(0);
    // Constructs a node (a.k.a., server, site, etc) set.
    let mut nodes = RendezvousNodes::new(hasher);
    nodes.insert("foo");
    nodes.insert("bar");
    nodes.insert("baz");
    nodes.insert("qux");

    // Finds candidate nodes for an item (a.k.a., object).
    assert_eq!(
        nodes.calc_candidates(&1).collect::<Vec<_>>(),
        [&"qux", &"baz", &"foo", &"bar"]
    );
    assert_eq!(
        nodes.calc_candidates(&"key").collect::<Vec<_>>(),
        [&"qux", &"baz", &"foo", &"bar"]
    );

    // Update the node set.
    // (The relative order between existing nodes are preserved)
    nodes.remove(&"baz");
    assert_eq!(
        nodes.calc_candidates(&1).collect::<Vec<_>>(),
        [&"qux", &"foo", &"bar"]
    );
    assert_eq!(
        nodes.calc_candidates(&"key").collect::<Vec<_>>(),
        [&"qux", &"foo", &"bar"]
    );

    let hasher = Mummur3NodeHasher::new(0);
    // Constructs a node (a.k.a., server, site, etc) set.
    let mut nodes = RendezvousNodes::new(hasher);
    nodes.insert("qux");
    nodes.insert("foo");
    nodes.insert("bar");
    nodes.insert("baz");

    // Finds candidate nodes for an item (a.k.a., object).
    assert_eq!(
        nodes.calc_candidates(&1).collect::<Vec<_>>(),
        [&"qux", &"baz", &"foo", &"bar"]
    );
    assert_eq!(
        nodes.calc_candidates(&"key").collect::<Vec<_>>(),
        [&"qux", &"baz", &"foo", &"bar"]
    );

    // Update the node set.
    // (The relative order between existing nodes are preserved)
    nodes.remove(&"baz");
    assert_eq!(
        nodes.calc_candidates(&1).collect::<Vec<_>>(),
        [&"qux", &"foo", &"bar"]
    );
    assert_eq!(
        nodes.calc_candidates(&"key").collect::<Vec<_>>(),
        [&"qux", &"foo", &"bar"]
    );
}

#[test]
fn rendezvous_hash_distribution_with_n_keys() {
    let key_counts = [10_000, 50_000, 100_000];

    let hasher = Mummur3NodeHasher::new(0);
    // Constructs a node (a.k.a., server, site, etc) set.
    let mut nodes = RendezvousNodes::new(hasher);
    nodes.insert("Node1");
    nodes.insert("Node2");
    nodes.insert("Node3");
    nodes.insert("Node4");

    for &num_keys in &key_counts {
        println!("Simulate distributing {:?} keys across the nodes", num_keys);

        let mut distribution = std::collections::HashMap::new();
        distribute_keys(&nodes, num_keys, &mut distribution);

        let expected_percentage = 100.0 / distribution.len() as f64;
        let margin_of_error = 5.0; // Allow a 5% margin of error

        for &count in distribution.values() {
            let actual_percentage = (count as f64 / num_keys as f64) * 100.0;
            assert!(
                (actual_percentage - expected_percentage).abs() <= margin_of_error,
                "Actual percentage {} is not within the margin of error for expected percentage {}",
                actual_percentage,
                expected_percentage
            );
        }

        println!();
    }
}

pub fn distribute_keys<'a>(
    nodes: &'a RendezvousNodes<&str, Mummur3NodeHasher>,
    num_keys: usize,
    distribution: &mut std::collections::HashMap<Option<&'a str>, usize>,
) {
    for i in 0..num_keys {
        let key = format!("Key{}", i);
        let node = match nodes.calc_candidates(&key).next() {
            Some(node) => Some(*node),
            None => None,
        };
        *distribution.entry(node).or_insert(0) += 1;
    }

    // Print out the distribution
    for (node, count) in distribution {
        println!(
            "{}: {} keys ({:.2}%)",
            node.unwrap(),
            count,
            (*count as f64 / num_keys as f64) * 100.0
        );
    }
}
