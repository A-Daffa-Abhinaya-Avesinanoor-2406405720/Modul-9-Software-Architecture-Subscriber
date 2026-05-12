# Refleksi Subscriber

## a. What is AMQP?

AMQP adalah singkatan dari Advanced Message Queuing Protocol. AMQP adalah aturan komunikasi yang dipakai aplikasi untuk mengirim dan menerima pesan melalui message broker seperti RabbitMQ.

Dalam arsitektur event-driven, publisher tidak mengirim data langsung ke subscriber. Publisher mengirim event ke message broker, lalu subscriber mengambil dan memproses event tersebut dari broker. AMQP membantu proses ini agar pengiriman pesan lebih teratur, aman, dan bisa menggunakan queue.

## b. What does `guest:guest@localhost:5672` mean?

Pada URL `amqp://guest:guest@localhost:5672`, bagian `guest:guest@localhost:5672` berisi informasi untuk terhubung ke RabbitMQ.

`guest` pertama adalah username yang digunakan untuk login ke RabbitMQ.

`guest` kedua adalah password dari username tersebut.

`localhost:5672` berarti RabbitMQ berjalan di komputer lokal, yaitu komputer yang sedang menjalankan program ini. `5672` adalah port default yang digunakan RabbitMQ untuk koneksi AMQP.

Jadi, URL tersebut artinya program subscriber akan mencoba terhubung ke RabbitMQ di komputer sendiri melalui port `5672`, menggunakan username `guest` dan password `guest`.
    