# Changelog

## v0.2.0

### Breaking changes

Add support for multi threaded socket acceptor and initiator.

To create socket acceptor and initiator you must change below code:

```diff
- let mut acceptor = SocketInitiator::try_new(&settings, &app, &store_factory, &log_factory)?;
+ let mut acceptor = Initiator::try_new(
+     &settings,
+     &app,
+     &store_factory,
+     &log_factory,
+     FixSocketServerKind::SingleThreaded,
+ )?;

- let mut initiator = SocketInitiator::try_new(&settings, &app, &message_store, &logger)?;
+ let mut initiator = Initiator::try_new(
+     &settings,
+     &app,
+     &message_store,
+     &logger,
+     FixSocketServerKind::default(),
+ )?;
```
