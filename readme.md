### Understanding subscriber and message broker

**a. What is amqp?**
AMQP (Advanced Message Queuing Protocol) is basically an open standard application layer protocol for message-oriented middleware. It defines how messages are sent, routed, and received between publishers, brokers, and subscribers.

**b. What does it mean? guest:guest@localhost:5672**
- The first `guest` is the default username for the RabbitMQ server.
- The second `guest` is the default password.
- `localhost:5672` is the host and port where the RabbitMQ message broker is running and listening for connections.