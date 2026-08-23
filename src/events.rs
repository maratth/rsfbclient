//! Firebird remote events API

use crate::{Connection, SimpleConnection};
use rsfbclient_core::{FbError, FirebirdClient, FirebirdClientDbEvents};
use std::thread::{self, JoinHandle};

/// Firebird remote events manager
pub trait RemoteEventsManager<F, C>
where
    F: FnMut(&mut SimpleConnection) -> Result<bool, FbError> + Send + Sync + 'static,
    C: FirebirdClient + FirebirdClientDbEvents + 'static,
    SimpleConnection: From<Connection<C>>,
{
    /// Start the event listener on a thread
    ///
    /// The event can be called many times.
    ///
    /// Stop when the handler return a error. Return a false
    /// value if you want stop the listner,
    fn listen_event(
        self,
        name: String,
        handler: F,
    ) -> Result<JoinHandle<Result<(), FbError>>, FbError>;
}

impl<F, C> RemoteEventsManager<F, C> for Connection<C>
where
    C: FirebirdClient + FirebirdClientDbEvents + 'static,
    F: FnMut(&mut SimpleConnection) -> Result<bool, FbError> + Send + Sync + 'static,
    SimpleConnection: From<Connection<C>>,
{
    fn listen_event(
        self,
        name: String,
        handler: F,
    ) -> Result<JoinHandle<Result<(), FbError>>, FbError> {
        let conn: SimpleConnection = self.into();

        return conn.listen_event(name, handler);
    }
}

impl<F, C> RemoteEventsManager<F, C> for SimpleConnection
where
    C: FirebirdClient + FirebirdClientDbEvents + 'static,
    F: FnMut(&mut SimpleConnection) -> Result<bool, FbError> + Send + Sync + 'static,
    SimpleConnection: From<Connection<C>>,
{
    fn listen_event(
        mut self,
        name: String,
        mut handler: F,
    ) -> Result<JoinHandle<Result<(), FbError>>, FbError> {
        return Ok(thread::spawn(move || {
            let mut hold = true;

            while hold {
                self.wait_for_event(name.clone())?;

                hold = handler(&mut self)?;
            }

            Ok(())
        }));
    }
}

#[cfg(test)]
mk_tests_default! {
    use crate::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use std::thread;

    #[test]
    #[ignore]
    #[cfg(not(feature = "embedded_tests"))]
    fn remote_events() -> Result<(), FbError> {
        let conn1 = cbuilder().connect()?;
        let mut conn2 = cbuilder().connect()?;

        let counter = Arc::new(Mutex::new(0)).clone();

        let acounter = Arc::clone(&counter);
        let _ = conn1.listen_event("evento".to_string(), move |c| {

            let (_,): (i32,) = c.query_first(
                "select 1 from rdb$database",
                (),
            )?.unwrap();

            let mut num = acounter.lock().unwrap();
            *num += 1;

            Ok(*num < 2)
        })?;

        thread::sleep(Duration::from_secs(2));

        conn2.execute("execute block as begin POST_EVENT 'evento'; end", ())?;
        thread::sleep(Duration::from_secs(2));
        assert_eq!(1, *counter.lock().unwrap());

        conn2.execute("execute block as begin POST_EVENT 'evento'; end", ())?;
        thread::sleep(Duration::from_secs(2));
        assert_eq!(2, *counter.lock().unwrap());

        conn2.execute("execute block as begin POST_EVENT 'evento'; end", ())?;
        thread::sleep(Duration::from_secs(2));
        assert_eq!(2, *counter.lock().unwrap());

        Ok(())
    }

    // `RemoteEventsManager` carries a client type that `SimpleConnection` does
    // not constrain, so this call only compiles when a single client
    // implementation is built in. Skipped otherwise, as it was before the pure
    // rust client learned about events.
    #[test]
    #[ignore]
    #[cfg(all(
        not(feature = "embedded_tests"),
        not(all(feature = "native_client", feature = "pure_rust")),
        not(all(feature = "linking", feature = "dynamic_loading"))
    ))]
    fn remote_events_simple_conn() -> Result<(), FbError> {
        let conn1: SimpleConnection = cbuilder().connect()?.into();
        let mut conn2: SimpleConnection = cbuilder().connect()?.into();

        let counter = Arc::new(Mutex::new(0)).clone();

        let acounter = Arc::clone(&counter);
        let _ = conn1.listen_event("evento2".to_string(), move |c| {

            let (_,): (i32,) = c.query_first(
                "select 1 from rdb$database",
                (),
            )?.unwrap();

            let mut num = acounter.lock().unwrap();
            *num += 1;

            Ok(*num < 2)
        })?;

        thread::sleep(Duration::from_secs(2));

        conn2.execute("execute block as begin POST_EVENT 'evento2'; end", ())?;
        thread::sleep(Duration::from_secs(2));
        assert_eq!(1, *counter.lock().unwrap());

        Ok(())
    }

    #[test]
    #[ignore]
    #[cfg(not(feature = "embedded_tests"))]
    fn wait_for_event() -> Result<(), FbError> {

        let mut conn1 = cbuilder().connect()?;
        let mut conn2 = cbuilder().connect()?;

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));

            conn1.execute("execute block as begin POST_EVENT 'evento3'; end", ())
        });

        let wait = thread::spawn(move || {
            conn2.wait_for_event("evento3".to_string())
        });

        thread::sleep(Duration::from_secs(10));
        assert!(wait.is_finished());

        Ok(())
    }

    /// A registration is one shot, so waiting twice on the same connection has
    /// to register twice. The second wait also reuses the auxiliary channel
    /// opened by the first one.
    #[test]
    #[ignore]
    #[cfg(not(feature = "embedded_tests"))]
    fn wait_for_event_twice() -> Result<(), FbError> {

        let mut listener = cbuilder().connect()?;
        let mut poster = cbuilder().connect()?;

        let waits = thread::spawn(move || -> Result<(), FbError> {
            listener.wait_for_event("evento4".to_string())?;
            listener.wait_for_event("evento4".to_string())?;

            Ok(())
        });

        thread::sleep(Duration::from_secs(2));
        poster.execute("execute block as begin POST_EVENT 'evento4'; end", ())?;

        thread::sleep(Duration::from_secs(2));
        assert!(!waits.is_finished(), "the second wait returned without an event");

        poster.execute("execute block as begin POST_EVENT 'evento4'; end", ())?;

        thread::sleep(Duration::from_secs(2));
        assert!(waits.is_finished(), "the second event was not delivered");
        waits.join().expect("the listener thread panicked")?;

        Ok(())
    }

    /// The server matches event names on their raw bytes, so a name has to be
    /// encoded with the charset of the connection, the one `POST_EVENT` reaches
    /// it with. Posting the same name from a utf-8 connection does not wake a
    /// listener registered from a latin 1 one.
    ///
    /// Only the pure rust client does this: the native one always sends the
    /// utf-8 bytes of the rust string, so the test is skipped when it is built
    /// in.
    #[test]
    #[ignore]
    #[cfg(all(
        feature = "pure_rust",
        not(feature = "native_client"),
        not(feature = "embedded_tests")
    ))]
    fn wait_for_event_with_a_non_utf8_charset() -> Result<(), FbError> {

        let name = "ev\u{e9}nement";

        let mut builder = cbuilder();
        builder.charset(charset::ISO_8859_1);

        let mut listener = builder.connect()?;
        let mut poster = builder.connect()?;

        let waits = thread::spawn(move || listener.wait_for_event(name.to_string()));

        thread::sleep(Duration::from_secs(2));
        poster.execute(
            &format!("execute block as begin POST_EVENT '{}'; end", name),
            (),
        )?;

        thread::sleep(Duration::from_secs(3));
        assert!(waits.is_finished(), "the latin 1 event was not delivered");
        waits.join().expect("the listener thread panicked")?;

        Ok(())
    }
}
