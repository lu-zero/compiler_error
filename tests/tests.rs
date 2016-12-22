#![feature(plugin)]
#![plugin(compiler_error)]


macro_rules! testme {
    ( error ) => {
        compiler_error!("test!");
    };
    ( ok ) => {
        println!("Ok");
    }
}

#[test]
fn test_macro() {
    testme!(ok);
    //testme!(error);
}
