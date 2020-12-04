//! Jupiter primitives - IO Module
#![allow(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime_interface::runtime_interface;

macro_rules! zk {
    ($($curve:ident),*) => {
        /// ZK-SNARKs runtime interface
        #[runtime_interface]
        pub trait ZkSnarks {
            $(
                paste::item!{
                    fn [<$curve _add>] () {
                        megaclite::tests::$curve_add();
                    }

                    fn [<$curve _mul>] () {
                        megaclite::tests::$curve_mul();
                    }

                    fn [<$curve _pairing_two>]() {
                        megaclite::tests::$curve_pairing();
                    }

                    fn [<$curve _pairing_six()>] {
                        megaclite::tests::$curve_pairing_six();
                    }
                }
            )*
        }
    };
}

zk! {
    bls12_377,
    bls12_381,
    bn254,
    bw6_761,
    cp6_782
}
