//! Crate tests and test utils

/// Generate copies of tests for multiple client implementations
macro_rules! mk_tests {
    // Base case
    (
        tests {
            $( $tests:tt )*
        }
    ) => {};

    // Recurse for each module
    (
        tests {
            $( $tests:tt )*
        }

        $( #[$attr:meta] )*
        for $name:ident -> $type:ty {
            $( $cbuilder:tt )*
        }

        $( $tail:tt )*
    ) => {
        $( #[$attr] )*
        mod $name {
            $( $tests )*

            // Used by the IntoParams derive
            #[allow(unused_imports)]
            use crate as rsfbclient;

            #[allow(dead_code)]
            fn cbuilder() -> $type {
                $( $cbuilder )*
            }
        }

        mk_tests! {
            tests {
                $( $tests )*
            }
            $( $tail )*
        }
    };
}

/// Generate copies of tests for the default client implementations
macro_rules! mk_tests_default {
    ( $( $tests:tt )* ) => {
        mk_tests! {
            tests {
                use crate::builders::*;


                $( $tests )*
            }

            #[cfg(all(feature = "linking", not(feature = "embedded_tests")))]
            for linking -> NativeConnectionBuilder<DynLink, ConnRemote> {
                crate::builder_native()
                  .with_dyn_link()
                  .with_remote()
            }

            #[cfg(all(feature = "linking", feature = "embedded_tests"))]
            for linking_embedded -> NativeConnectionBuilder<DynLink, ConnEmbedded> {
                crate::builder_native()
                    .with_dyn_link()
                    .with_embedded()
                    .db_name("/tmp/embedded_tests.fdb")
                    .clone()
            }

            #[cfg(all(feature = "dynamic_loading", not(feature = "embedded_tests")))]
            for dynamic_loading -> NativeConnectionBuilder<DynLoad, ConnRemote> {

                #[cfg(target_os = "linux")]
                let libfbclient = "libfbclient.so";
                #[cfg(target_os = "windows")]
                let libfbclient = "fbclient.dll";
                #[cfg(target_os = "macos")]
                let libfbclient = "libfbclient.dylib";

                crate::builder_native()
                  .with_dyn_load(libfbclient)
                  .with_remote()
            }

            #[cfg(all(feature = "dynamic_loading", feature = "embedded_tests"))]
            for dynamic_loading_embedded -> NativeConnectionBuilder<DynLoad, ConnEmbedded> {

                #[cfg(target_os = "linux")]
                let libfbclient = "libfbclient.so";
                #[cfg(target_os = "windows")]
                let libfbclient = "fbclient.dll";
                #[cfg(target_os = "macos")]
                let libfbclient = "libfbclient.dylib";

                crate::builder_native()
                    .with_dyn_load(libfbclient)
                    .with_embedded()
                    .db_name("/tmp/embedded_tests.fdb")
                    .clone()
            }

            #[cfg(feature = "pure_rust")]
            for pure_rust -> PureRustConnectionBuilder {
                crate::builder_pure_rust()
            }
        }
    };
}

mod charset;
mod connection;
mod database;
mod params;
mod row;
mod transaction;
mod service;
