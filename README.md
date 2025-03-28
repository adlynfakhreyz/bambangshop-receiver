# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [v] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [v] Commit: `Create Notification model struct.`
    -   [v] Commit: `Create SubscriberRequest model struct.`
    -   [v] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [v] Commit: `Implement add function in Notification repository.`
    -   [v] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [v] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [v] Commit: `Create Notification service struct skeleton.`
    -   [v] Commit: `Implement subscribe function in Notification service.`
    -   [v] Commit: `Implement subscribe function in Notification controller.`
    -   [v] Commit: `Implement unsubscribe function in Notification service.`
    -   [v] Commit: `Implement unsubscribe function in Notification controller.`
    -   [v] Commit: `Implement receive_notification function in Notification service.`
    -   [v] Commit: `Implement receive function in Notification controller.`
    -   [v] Commit: `Implement list_messages function in Notification service.`
    -   [v] Commit: `Implement list function in Notification controller.`
    -   [v] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1

1. Penggunaan `RwLock<>` untuk sinkronisasi koleksi `Vec` yang menyimpan `Notification` sangat penting karena karakteristik dari penggunaan data kita. `RwLock<>` memungkinkan beberapa thread untuk membaca data secara bersamaan selama tidak ada thread yang sedang menulis. Ini berbeda dengan `Mutex<>` yang hanya mengizinkan satu thread pada satu waktu untuk mengakses data, baik untuk operasi baca maupun tulis. Dalam konteks aplikasi penerima notifikasi, kita memiliki skenario dimana akan ada banyak operasi pembacaan notifikasi oleh berbagai thread yang berbeda, namun operasi penulisan (penambahan notifikasi baru) relatif lebih jarang. Dengan menggunakan `RwLock<>`, kita mendapatkan peningkatan performa karena memungkinkan pembacaan paralel tanpa mengorbankan keamanan thread saat menulis data.

2. Rust memiliki pendekatan yang sangat berbeda dengan Java dalam hal variabel static. Di Rust, variabel statisc secara default bersifat immutable untuk menjamin keamanan dalam lingkungan multithreading. Ini merupakan bagian dari filosofi keamanan memori dan konkurensi yang diusung Rust. Berbeda dengan Java yang memungkinkan kita memodifikasi variabel static melalui fungsi static, Rust membatasi kemampuan ini untuk mencegah race condition dan masalah konkurensi lainnya. Penggunaan library `lazy_static` memberikan kita cara untuk mendefinisikan variabel static yang dapat dimodifikasi (mutable) dengan aman, sambil tetap mempertahankan pola Singleton—memastikan hanya ada satu instance dari variabel tersebut selama aplikasi berjalan. `lazy_static` juga memungkinkan inisialisasi yang ditunda (lazy) sehingga variabel hanya dibuat saat pertama kali diakses, menghemat sumber daya sistem.

#### Reflection Subscriber-2

1. Sudaah, File ini berfungsi sebagai pusat konfigurasi aplikasi yang berisi definisi penting seperti REQWEST_CLIENT untuk HTTP request dan APP_CONFIG untuk pengaturan aplikasi dengan pola Singleton. Di dalamnya juga terdapat sistem penanganan error terpusat dengan ErrorResponse dan fungsi compose_error_response, serta penggunaan library seperti lazy_static, dotenv, dan getset. Eksplorasi ini membantu saya memahami bagaimana aplikasi mengelola konfigurasi dan penanganan error secara terstruktur.

2. Design pattern Observer sangat mempermudah penambahan subscriber baru. Ini karena pola Observer yang diimplementasikan mematuhi prinsip Open-Closed, di mana sistem terbuka untuk ekstensi tanpa perlu memodifikasi kode yang sudah ada. Ketika kita ingin menambahkan subscriber baru, cukup dengan menjalankan instance baru dan mendaftarkannya ke sistem. Mengenai penggunaan lebih dari satu instance Main app, hal ini tetap bisa dilakukan dengan mudah - kita hanya perlu mendaftarkan subscriber ke masing-masing aplikasi Main dengan cara mengirimkan request API ke endpoint yang sesuai. Fleksibilitas ini menunjukkan keunggulan dari penerapan pola Observer.

3. Dengan Postman, saya bisa menguji endpoint aplikasi secara langsung dan melihat respons yang diberikan. Fitur collection memungkinkan saya untuk mengorganisir dan menyimpan berbagai request yang sering digunakan, sehingga tidak perlu membuat ulang request yang sama berkali-kali. Ditambah lagi, kemampuan Postman untuk bekerja dengan data aktual aplikasi, memungkinkan saya untuk memverifikasi bahwa aplikasi merespons dengan benar untuk berbagai skenario input.