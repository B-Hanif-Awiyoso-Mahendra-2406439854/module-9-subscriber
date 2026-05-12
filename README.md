# Subscriber

## Understanding subscriber and message broker

### Apa itu AMQP?

AMQP adalah singkatan dari Advanced Message Queuing Protocol. AMQP merupakan protokol yang digunakan oleh message broker, seperti RabbitMQ, agar beberapa aplikasi dapat saling bertukar pesan melalui queue.

### Apa arti `guest:guest@localhost:5672`?

Pada `amqp://guest:guest@localhost:5672`, `guest` pertama adalah username, `guest` kedua adalah password, sedangkan `localhost:5672` menunjukkan bahwa RabbitMQ berjalan di komputer lokal pada port `5672`.
