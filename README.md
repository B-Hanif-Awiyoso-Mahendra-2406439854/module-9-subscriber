# Subscriber

## Understanding subscriber and message broker

### Apa itu AMQP?

AMQP adalah singkatan dari Advanced Message Queuing Protocol. AMQP merupakan protokol yang digunakan oleh message broker, seperti RabbitMQ, agar beberapa aplikasi dapat saling bertukar pesan melalui queue.

### Apa arti `guest:guest@localhost:5672`?

Pada `amqp://guest:guest@localhost:5672`, `guest` pertama adalah username, `guest` kedua adalah password, sedangkan `localhost:5672` menunjukkan bahwa RabbitMQ berjalan di komputer lokal pada port `5672`.

## Simulation slow subscriber

Pada simulasi ini, subscriber dibuat lebih lambat dengan menambahkan delay 1 detik saat memproses setiap message.

```rust
std::thread::sleep(_ten_millis);
```

Ketika publisher dijalankan beberapa kali, message yang masuk ke RabbitMQ lebih cepat daripada kemampuan subscriber untuk memprosesnya. Akibatnya, message sempat menumpuk pada queue `user_created`. Kondisi ini dapat dilihat melalui RabbitMQ Management pada grafik `Queued messages`.

![alt text](image.png)

## Reflection and Running at Least Three Subscribers

Pada percobaan ini, saya menjalankan tiga subscriber secara bersamaan menggunakan Docker Compose.

```powershell
docker compose up --scale subscriber=3 rabbitmq subscriber
```

Setelah itu, publisher dijalankan beberapa kali untuk mengirim banyak event ke RabbitMQ.

```powershell
docker compose run --rm publisher
```

Hasilnya, message pada queue `user_created` diproses oleh lebih dari satu subscriber. Hal ini terlihat dari log terminal yang menunjukkan beberapa instance subscriber menerima message. Dengan tiga subscriber, proses konsumsi message menjadi lebih cepat karena RabbitMQ membagikan message ke beberapa consumer yang berjalan paralel.

![alt text](image-1.png)
![alt text](image-2.png)
