mod front_of_house {
    mod hosting {
        fn add_to_waiting_list() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// module tree for the above structure
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waiting_list
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/
