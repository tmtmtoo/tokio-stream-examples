# `Fan-Out/Fan-In` Example
An implementation with the following model.

```
 
                                   +------------------+
                                   | Transform        |
                                   +------------------+
                             +-----> ()        String +----+
      +-----------------+    |     |                  |    |    +-----------------+
      | Source          |    |     |     Counter      |    |    | Sink            |
      +-----------------+    |     |                  |    |    +-----------------+
      |         Instant +----+     +------------------+    +----> String          |
      |                 |    |     +------------------+    |    |                 |
      |      Timer      |    |     | Transform        |    |    |     Stdout      |
      |                 |    |     +------------------+    |    |                 |
      +-----------------+    +-----> Instant   String +----+    +-----------------+
                                   |                  |
                                   |   Elapsed Time   |
                                   |                  |
                                   +------------------+

```

- Timer is a stream source, periodically outputs timer instants.
- Counter is a stream transformer. It counts timer with internal state.
- ElapsedTime is a stream transformer. It measurements elapsed timesince the timer started.

