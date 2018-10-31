#[macro_use]
extern crate getset;

use crate::submodule::another::other::Plain;
// For testing `pub(in submodule)`
mod submodule {
    // For testing `pub(in super)`
    pub mod another {
        pub mod other {
            #[derive(Getters, Setters)]
            #[get]
            #[set]
            pub struct Plain {
                /// A doc comment.
                /// Multiple lines, even.
                private_accessible: usize,

                /// A doc comment.
                #[get = "pub"]
                #[set = "pub"]
                public_accessible: usize,
                /// A doc comment.
                #[get = "pub(crate)"]
                #[set = "pub(crate)"]
                crate_accessible: usize,

                /// A doc comment.
                #[get(vis = "pub(super)")]
                #[set(vis = "pub(super)")]
                super_accessible: usize,

                /// A doc comment.
                #[get(vis = "pub(in crate::submodule)")]
                #[set(vis = "pub(in crate::submodule)")]
                scope_accessible: usize,
            }

            impl Default for Plain {
                fn default() -> Plain {
                    Plain {
                        private_accessible: 17,
                        public_accessible: 18,
                        crate_accessible: 19,
                        super_accessible: 20,
                        scope_accessible: 21,
                    }
                }
            }

            #[test]
            fn test_private_accessible() {
                let mut val = Plain::default();
                val.private_accessible();
                val.set_private_accessible(1);
            }
        }
        #[test]
        fn test_super_accessible() {
            use self::other::Plain;
            let mut val = Plain::default();
            val.super_accessible();
            val.set_super_accessible(1);
        }
    }
    #[test]
    fn test_scope_accessible() {
        use self::another::other::Plain;
        let mut val = Plain::default();
        val.scope_accessible();
        val.set_scope_accessible(1);
    }
}

#[test]
fn test_public_accessible() {
    let mut val = Plain::default();
    val.public_accessible();
    val.set_public_accessible(1);
}

#[test]
fn test_crate_accessible() {
    let mut val = Plain::default();
    val.crate_accessible();
    val.set_crate_accessible(1);
}
