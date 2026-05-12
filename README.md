# Subscriber

## Understanding subscriber and message broker

### Apa itu AMQP?

AMQP adalah singkatan dari Advanced Message Queuing Protocol. AMQP merupakan protokol yang digunakan oleh message broker, seperti RabbitMQ, agar beberapa aplikasi dapat saling bertukar pesan melalui queue.

### Apa arti `guest:guest@localhost:5672`?

Pada `amqp://guest:guest@localhost:5672`, `guest` pertama adalah username, `guest` kedua adalah password, sedangkan `localhost:5672` menunjukkan bahwa RabbitMQ berjalan di komputer lokal pada port `5672`.

## Simulation slow subscriber
Pada simulasi ini, subscriber dibuat lebih lambat dengan menambahkan delay 1 detik saat memproses setiap message.
![alt text](image.png)

## Reflection and Running at Least Three Subscribers

Pada percobaan ini, saya menjalankan tiga subscriber secara bersamaan menggunakan Docker Compose.
![alt text](image-1.png)
![alt text](image-2.png)
