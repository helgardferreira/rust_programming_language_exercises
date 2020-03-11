/*
Rust's memory safety guarantees make it difficult, but not impossible, to
accidentally create memory that is never cleaned up (known as a memory leak).
Preventing memory leaks entirely is not one of Rust's guarantees in the same way
that disallowing data races at compile time is, meaning memory leaks are memory
safe in Rust. We can see that Rust allows memory leaks by using Rc<T> and
RefCell<T>: it's possible to create references where items refer to each other in
a cycle. This creates memory leaks because the reference count of each item in the
cycle will never reach 0, and the values will never be dropped.
*/

use crate::List::{Cons, Nil};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// list type that is capable of reference cycle
// (since it is a self-referential type)
#[derive(Debug)]
enum List {
    /*
    The second element in the Cons variant is RefCell<Rc<List>>, meaning that
    we want to modify which List value a Cons variant is pointing to.
    */
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // tail method to grab second entry in List (if the enum variant is Cons)
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(
        Cons(
            5,
            RefCell::new(Rc::new(Nil)),
        )
    );

    println!("a's initial rc count = {}", Rc::strong_count(&a));
    println!("a's next item = {:?}", a.tail());

    if let Some(link) = a.tail() {
        println!("a's next next item = {:?}", link.borrow().tail());
    }

    let b = Rc::new(
        Cons(
            10,
            RefCell::new(Rc::clone(&a)),
        )
    );

    println!("a's rc count after b's creation = {}", Rc::strong_count(&a));
    println!("b's initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b's rc count after changing a = {}", Rc::strong_count(&b));
    println!("a's rc count after changing a = {}", Rc::strong_count(&a));

    /*
    The reference count of the Rc<List> instances in both a and b are 2 after we
    change the list in a to point to b. At the end of main, Rust will try to drop
    b first, which will decrease the count of the Rc<List> instance in b by 1.

    However, because a is still referencing the Rc<List> that was in b, that
    Rc<List> has a count of 1 rather than 0, so the memory the Rc<List> has on
    the heap won't be dropped. The memory will just sit there with a count of 1,
    forever.
    */

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    // making use of the Node struct
    /*let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });*/

    // leaf now has two owners - leaf (itself) and branch

    /*println!("{:?}", leaf);
    println!("{:?}", branch);*/

    /*
    We can get from branch to leaf through branch.children, but there's no way
    to get from leaf to branch. The reason is that leaf has no reference to
    branch and doesn't know they're related. We want leaf to know that branch is
    its parent. We'll do that next.
    */

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    /*let val = leaf.parent.borrow().upgrade();
    match val {
        Some(node_ref) => {
            println!("{:?}", node_ref
                .children.borrow()[0]
                .children.borrow()
                .len()
            );
        }
        None => {}
    }*/

    println!("------------------------------------------------------------");

    // visualizing changes to strong_count and weak_count
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        /*let val = if let Some(node_ref) = leaf.parent
            .borrow_mut()
            .upgrade() {
            let x = Rc::clone(&node_ref);
            x
        } else {
            Rc::new(Node {
                children: RefCell::new(vec![]),
                parent: RefCell::new(Weak::new()),
                value: 0,
            })
        };*/

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // explicitly dropping branch with the drop function
    // drop(branch);

    // branch goes out of scope here and is thus de-allocated

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    /*
    By specifying that the relationship from a child to its parent should be a
    Weak<T> reference in the definition of Node, you're able to have parent nodes
    point to child nodes and vice versa without creating a reference cycle and
    memory leaks!
    */
}

/*
Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

So far, we've demonstrated that calling Rc::clone increases the strong_count of
an Rc<T> instance, and an Rc<T> instance is only cleaned up if its strong_count
is 0. You can also create a weak reference to the value within an Rc<T> instance
by calling Rc::downgrade and passing a reference to the Rc<T>. When you call
Rc::downgrade, you get a smart pointer of type Weak<T>. Instead of increasing the
strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the
weak_count by 1. The Rc<T> type uses weak_count to keep track of how manyWeak<T>
references exist, similar to strong_count. The difference is the weak_count
doesn't need to be 0 for the Rc<T> instance to be cleaned up.

Strong references are how you can share ownership of an Rc<T> instance. Weak
references don't express an ownership relationship. They won't cause a reference
cycle because any cycle involving some weak references will be broken once the
strong reference count of values involved is 0.

Because the value that Weak<T> references might have been dropped, to do anything
with the value that a Weak<T> is pointing to, you must make sure the value still
exists. Do this by calling the upgrade method on a Weak<T> instance, which will
return an Option<Rc<T>>. You'll get a result of Some if the Rc<T> value has not
been dropped yet and a result of None if the Rc<T> value has been dropped.
Because upgrade returns an Option<T>, Rust will ensure that the Some case and
the None case are handled, and there won't be an invalid pointer.
*/

// creating a tree data structure: a node with child nodes
#[derive(Debug)]
struct Node {
    // ( Mutable Reference ( Collection of Reference Counters ( Node ) ) )
    children: RefCell<Vec<Rc<Node>>>,
    /*
    To make the child node aware of its parent, we need to add a parent field to
    our Node struct definition. The trouble is in deciding what the type of
    parent should be. We know it can't contain an Rc<T>, because that would
    create a reference cycle with leaf.parent pointing to branch and
    branch.children pointing to leaf, which would cause their strong_count values
    to never be 0.

    Thinking about the relationships in another way, a parent node should own its
    children: if a parent node is dropped, its child nodes should be dropped as
    well. However, a child should not own its parent: if we drop a child node,
    the parent should still exist. This is a case for weak references!
    */
    parent: RefCell<Weak<Node>>,
    value: i32,
}
