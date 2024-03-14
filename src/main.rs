mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()` ");
    }
    pub fn function() {
        println!("called `my_mod::function()`");
    }
    pub fn indirect_access() {
        println!("called `my_mod::indirect_function()`, that ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()` ");
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }
        pub (in create::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod()`, that");
            public_function_in_nested();
        }
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_my_nested()`");
        }
        pub(crate) fn public_function_in_crate() {
            println!("called `my_mod::nested::crate()`");
        } 
    }
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        } 
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `ny_mod::privated_nested::restricted_function()`");
        }
    }
}


fn function() {
    
}