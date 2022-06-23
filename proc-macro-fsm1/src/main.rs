use proc_macro_fsm1::{fsm1, fsm1_state};

fsm1!(
    struct MyFsm {
        a_i32: i32,
    }

    fn non_state_fn(& mut self) {
        self.a_i32 += 1;
        println!("MyFSM: non_state_fn self.data={}", self.a_i32);
    }

    #[fsm1_state]
    fn initial(& mut self) -> bool {
        self.non_state_fn();
        self.transition_to(MyFsm::do_work);
        println!("MyFSM: initial self.a_i32={}", self.a_i32);
        true
    }

    #[fsm1_state]
    fn do_work(& mut self) -> bool {
        self.a_i32 += 1;
        self.transition_to(MyFsm::done);
        println!("MyFSM: do_work self.a_i32={}", self.a_i32);
        true
    }

    #[fsm1_state]
    fn done(& mut self) -> bool {
        self.a_i32 += 1;
        println!("MyFSM: done self.a_i32={}", self.a_i32);
        true
    }
);


fn main() {
    // Verify new without type works
    let mut my_new_fsm = MyFsm::new();
    my_new_fsm.a_i32 = 321;
    assert_eq!(my_new_fsm.a_i32, 321);

    // Verify new with type works
    let mut my_new_fsm: MyFsm = MyFsm::new();
    my_new_fsm.a_i32 = 456;
    assert_eq!(my_new_fsm.a_i32, 456);

    // Verify default without type works
    let mut my_new_fsm = MyFsm::default();
    my_new_fsm.a_i32 = 213;

    // Verify default with type works
    let mut my_fsm: MyFsm = Default::default();
    //assert!(my_fsm.sm.current_state == MyFsm::initial, "current_state != MyFsm::initial");
    //assert!(my_fsm.sm.previous_state == MyFsm::initial, "previous_state != MyFsm::initial");
    assert!(my_fsm.sm.current_state_changed == true, "current_state_changed != true");

    my_fsm.a_i32 = 123;
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);

    // Invoke initial
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={} csc={}", my_fsm.a_i32, my_fsm.sm.current_state_changed);
    //assert!(my_fsm.sm.current_state == MyFsm::do_work, "current_state != MyFsm::do_work");
    //assert!(my_fsm.sm.previous_state == MyFsm::initial, "previous_state != MyFsm::initial");
    assert!(my_fsm.sm.current_state_changed == true, "current_state_changed != true");

    // Invoke do_work
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    //assert!(my_fsm.sm.current_state == MyFsm::done, "current_state != MyFsm::done");
    //assert!(my_fsm.sm.previous_state == MyFsm::do_work, "previous_state != MyFsm::do_work");
    assert!(my_fsm.sm.current_state_changed == true, "current_state_changed != true");

    // Invoke done
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    //assert!(my_fsm.sm.current_state == MyFsm::done, "current_state != MyFsm::done");
    //assert!(my_fsm.sm.previous_state == MyFsm::do_work, "previous_state != MyFsm::do_work");
    assert!(my_fsm.sm.current_state_changed == false, "current_state_changed != false");

    // Invoke done again
    my_fsm.dispatch();
    println!("main: my_fsm.a_i32={}", my_fsm.a_i32);
    //assert!(my_fsm.sm.current_state == MyFsm::done, "current_state != MyFsm::done");
    //assert!(my_fsm.sm.previous_state == MyFsm::do_work, "previous_state != MyFsm::do_work");
    assert!(my_fsm.sm.current_state_changed == false, "current_state_changed != false");
}

#[cfg(test)]
mod tests {
    use proc_macro_fsm1::{fsm1, fsm1_state};

    #[test]
    fn test_initialization_via_default() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                true
            }
        );

        let fsm: Test = Default::default();
        //assert!(fsm.sm.current_state == Test::initial, "current_state != Test1::initial");
        //assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
    }

    #[test]
    fn test_initialization_via_new() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                true
            }
        );

        let fsm = Test::new();
        //assert!(fsm.sm.current_state == Test::initial, "current_state != Test1::initial");
        //assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
    }

    #[test]
    fn test_transition_to() {
        fsm1!(
            struct Test {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                self.transition_to(Test::done);
                true
            }

            #[fsm1_state]
            fn done(& mut self) -> bool {
                true
            }
        );

        let mut fsm = Test::new();
        //assert!(fsm.sm.current_state == Test::initial, "current_state != Test1::initial");
        //assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
        fsm.sm.current_state_changed = false;
        _ = fsm.initial();
        //assert!(fsm.sm.current_state == Test::done, "current_state != Test1::done");
        //assert!(fsm.sm.previous_state == Test::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);
    }

    #[test]
    fn test_dispatch() {
        fsm1!(
            struct TestDispatch {}

            #[fsm1_state]
            fn initial(& mut self) -> bool {
                self.transition_to(TestDispatch::done);
                true
            }

            #[fsm1_state]
            fn done(& mut self) -> bool {
                true
            }
        );

        let mut fsm = TestDispatch::new();
        //assert!(fsm.sm.current_state == TestDispatch::initial, "current_state != Test1::initial");
        //assert!(fsm.sm.previous_state == TestDispatch::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);

        fsm.dispatch();
        //assert!(fsm.sm.current_state == TestDispatch::initial, "current_state != Test1::initial");
        //assert!(fsm.sm.previous_state == TestDispatch::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, true);

        fsm.dispatch();
        //assert!(fsm.sm.current_state == TestDispatch::done, "current_state != Test1::done");
        //assert!(fsm.sm.previous_state == TestDispatch::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, false);

        fsm.dispatch();
        //assert!(fsm.sm.current_state == TestDispatch::done, "current_state != Test1::done");
        //assert!(fsm.sm.previous_state == TestDispatch::initial, "previous_state != Test1::initial");
        assert_eq!(fsm.sm.current_state_changed, false);
    }
}
