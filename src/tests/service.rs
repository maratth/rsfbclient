//!
//! Rust Firebird Client
//!
//! Service tests
//!

mk_tests_default! {
    #[allow(unused_imports)]
    use crate::*;

    #[test]
    #[cfg(all(feature = "linking", not(feature = "embedded_tests"), not(feature = "pure_rust")))]
    fn string_conn1() -> Result<(), FbError> {
        builder_native()
            .from_string(
                "firebird://SYSDBA:masterkey@localhost:3050/service_mgr",
            )?
            .connect_service()?;

        builder_native()
            .from_string(
                "firebird://localhost:3050/service_mgr",
            )?
            .connect_service()?;

        Ok(())
    }

   #[test]
    #[cfg(all(feature = "dynamic_loading", not(feature = "embedded_tests"), not(feature = "pure_rust")))]
    fn string_conn2() -> Result<(), FbError> {

        #[cfg(target_os = "linux")]
        let libfbclient = "libfbclient.so";
        #[cfg(target_os = "windows")]
        let libfbclient = "fbclient.dll";
        #[cfg(target_os = "macos")]
        let libfbclient = "libfbclient.dylib";

        builder_native()
            .from_string(
                &format!("firebird://SYSDBA:masterkey@localhost:3050/service_mgr?lib={}", libfbclient),
            )?
            .connect_service()?;

        builder_native()
            .from_string(
                &format!("firebird://localhost:3050/service_mgr?lib={}", libfbclient),
            )?
            .connect_service()?;

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "linking", feature = "embedded_tests", not(feature = "dynamic_loading"), not(feature = "pure_rust")))]
    fn string_conn3() -> Result<(), FbError> {
        builder_native()
            .from_string(
                "firebird:///service_mgr",
            )?
            .connect_service()?;

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "dynamic_loading", feature = "embedded_tests", not(feature = "linking"), not(feature = "pure_rust")))]
    fn string_conn4() -> Result<(), FbError> {

        #[cfg(target_os = "linux")]
        let libfbclient = "libfbclient.so";
        #[cfg(target_os = "windows")]
        let libfbclient = "fbclient.dll";
        #[cfg(target_os = "macos")]
        let libfbclient = "libfbclient.dylib";

        builder_native()
            .from_string(
                &format!("firebird://service_mgr?lib={}", libfbclient),
            )?
            .connect_service()?;

        Ok(())
    }

    #[test]
    #[cfg(all(feature = "pure_rust", not(feature = "native_client")))]
    fn string_conn5() -> Result<(), FbError> {
        builder_pure_rust()
            .from_string(
                "firebird://SYSDBA:masterkey@localhost:3050/service_mgr?dialect=3",
            )?
            .connect_service()?;

        builder_pure_rust()
            .from_string(
                "firebird://localhost:3050/service_mgr",
            )?
            .connect_service()?;

        Ok(())
    }
}