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
