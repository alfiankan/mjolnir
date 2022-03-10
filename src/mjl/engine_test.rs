#[cfg(test)]
mod engine_tests {
    use owo_colors::OwoColorize;
    use crate::mjl::engine::Engine;

    #[test]
    fn read_persistent_file_test() {
        let mjl = Engine::new("data.mj");

        let result = mjl.read_persistent_data();

        println!("{:?}", result);

        match result {
            Ok(_) => {
                assert_eq!(true, true);
            }
            Err(_) => {
                assert_eq!(true, false);
            }
        }
    }

    #[test]
    fn genesis_test() {
        let mjl = Engine::new("data.mj");
        println!("{:?}", mjl.genesis().unwrap().on_green());
    }

    #[test]
    fn insert_to_chain() {
        let mjl = Engine::new("data.mj");
        let result = mjl.insert_to_chain("hello:world".to_string(), "c99b423478a6d0b7230aba2677ce094f7abdf38f40e40eaf322a863371f0eedf" );

        assert_eq!(true, result.is_ok(), "{:?}", result.err());
    }

    #[test]
    fn get_all_databox_as_chain_test() {
        let mjl = Engine::new("data.mj");
        match mjl.get_all_box_from_record("c99b423478a6d0b7230aba2677ce094f7abdf38f40e40eaf322a863371f0eedf".to_string()) {
            Ok(rows) => {
                for x in rows.boxs {
                    println!("{:?}", x);
                }
            }
            Err(_) => {}
        }
    }

    #[test]
    fn get_all_records_test() {
        let mjl = Engine::new("data.mj");
        println!("{:?}", mjl.list_records()  );
    }

    #[test]
    fn find_last_box_test() {
        let mjl = Engine::new("data.mj");
        println!("{:?}", mjl.find_last_box("c99b423478a6d0b7230aba2677ce094f7abdf38f40e40eaf322a863371f0eedf".to_string() ) );
    }

}