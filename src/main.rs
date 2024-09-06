mod array;
mod linkedlist;
mod stack;

fn main() {
    let mut linked = linkedlist::linked_list::LinkedList::new();

    linked.insert_end(1);
    linked.insert_end(4);
    linked.insert_end(6);
    linked.insert_end(3);
    linked.insert_end(2);

    linked.print();

    linked.remove(2)
}
