enum BinaryTree<T>{
	Empty,
	NonEmpty(Box<TreeNode<T>>)
}
struct TreeNode<T>{
	element: T,
	left: BinaryTree<T>,
	right: BinaryTree<T>,
}

use self::BinaryTree::*;
pub fn handle_planets(){

	let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode{
		element: "Jupiter",
		left: Empty,
		right: Empty
	}));
	let mercury_tree = BinaryTree::NonEmpty(Box::new(TreeNode{
		element: "Mercury",
		left: Empty,
		right: Empty
	}));
	let mars_tree = BinaryTree::NonEmpty(Box::new(TreeNode{
		element: "Mars",
		left: jupiter_tree,
		right: mercury_tree
	}));
	let venus_tree = BinaryTree::NonEmpty(Box::new(TreeNode{
		element: "Venus",
		left: Empty,
		right: Empty
	}));
	let uranus_tree = BinaryTree::NonEmpty(Box::new(TreeNode{
		element: "Uranus",
		left: Empty,
		right: venus_tree
	}));
	let saturn_tree = BinaryTree::NonEmpty(Box::new(TreeNode{
		element: "Saturn",
		left: mars_tree,
		right: uranus_tree
	}));
}