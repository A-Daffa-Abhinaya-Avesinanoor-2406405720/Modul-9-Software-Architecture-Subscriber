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

## Slow subscriber
![alt text](images/image.png)

Subscriber menjadi lebih lambat karena setiap message membutuhkan waktu sekitar 1 detik untuk diproses. Sementara itu, publisher tetap bisa mengirim banyak event dengan cepat setiap kali `cargo run` dijalankan. Karena publisher tidak perlu menunggu subscriber selesai memproses semua event, event-event tersebut akan masuk dulu ke RabbitMQ. RabbitMQ lalu menyimpan event yang belum sempat diproses di dalam queue. Itulah sebabnya jumlah queued message bisa naik ketika publisher dijalankan berkali-kali dalam waktu singkat.

Total queue menjadi sebesar itu karena jumlah event yang masuk lebih cepat daripada jumlah event yang keluar dari queue. Misalnya, jika satu kali publisher dijalankan mengirim 5 message, maka menjalankan publisher beberapa kali akan menambah banyak message sekaligus. Subscriber hanya mengambil dan memproses message satu per satu, ditambah delay 1 detik pada setiap message. Jadi, selama subscriber masih sibuk memproses message sebelumnya, message baru akan menunggu di queue. Setelah publisher berhenti mengirim event, jumlah queue akan turun pelan-pelan karena subscriber terus mengambil dan memproses message sampai habis.

## Multi subscriber
![alt text](images/image2.png)
![alt text](images/image3.png)

Ketika ada beberapa subscriber yang berjalan pada queue yang sama, RabbitMQ akan membagi message ke subscriber yang tersedia. Jadi, satu message hanya diproses oleh salah satu subscriber, bukan diproses ulang oleh semua subscriber. Karena itu, output di terminal bisa terbagi: ada subscriber yang memproses message seperti "Budi" dan "Dira", sementara subscriber lain memproses "Amir", "Cica", dan "Emir".

Hal ini terjadi karena RabbitMQ menggunakan queue sebagai tempat queue kerja. Setiap subscriber yang terhubung ke queue yang sama berperan seperti worker. Saat ada message baru, RabbitMQ akan mengirim message tersebut ke salah satu worker yang siap menerima pekerjaan. Dengan 3 subscriber, pekerjaan yang sebelumnya hanya dikerjakan oleh 1 subscriber sekarang bisa dibagi ke 3 proses subscriber.

Akibatnya, spike pada message queue turun lebih cepat dibandingkan saat hanya ada 1 subscriber. Kalau hanya ada 1 subscriber dan setiap message membutuhkan waktu sekitar 1 detik, maka semua message harus diproses satu per satu oleh satu proses saja. Tetapi ketika ada 3 subscriber, beberapa message bisa diproses secara paralel. Jumlah message yang menunggu di queue pun lebih cepat berkurang karena kapasitas proses consumer menjadi lebih besar.

Ini menunjukkan salah satu keuntungan event-driven architecture. Ketika producer mengirim request lebih cepat daripada consumer memprosesnya, sistem tidak langsung berhenti atau crash karena message masih bisa disimpan di queue. Jika queue mulai menumpuk, kita dapat menambah jumlah subscriber agar proses event menjadi lebih cepat dan beban kerja terbagi.
