## Modul 8

1. perbedaan unary, server streaming, dan bi-directional streaming RPC
unary => klien mengirim satu permintaan dan menerima satu respons dari server

server streaming => klien mengirim satu permintaan dan menerima beberapa respons dari server

bi-directional streaming => klien dan server komunikasi dua arah secara bersamaan


2. sekuritas yang perlu diconsider ketika menggunakan gRPC berupa: 
- autentikasi => verifikasi identitas klien/server. terdapat fitur di gRPC yang mendukung autentikasi seperti OAuth2, JSON Web TOkens(JWT)
- autorisasi => mengontrol akses ke resources berdasarkan peran/ izin. terdapat fitur seperti RBAC atau ABAC
- enskripsi data => gRPC menggunakan Transport Layer Security secara default. TLS digunakan untuk melindungi data selama transmisi


3. isu dan kesulitan yang mungkin muncul ketika handle bidirectional streaming di rust gRPC
- sinkronisasi stream klien dan server untuk konkurensi perlu async/await yang kompleks karena perlu mengelola dua stream secara bersamaan
- backpressure. salah satu pihak antara klien atau server mungkin menghasilkan data lebih cepat sebelum di proses sehingga terjadi kegagalan sistem
- kesalahan di salah satu stream memengaruhi stream lainnya
- kebocoran memori jika stream bidirectional tidak ditutup dengan benar
- kesulitan debugging dan pengamatan. interaksi antara klien dan server kompleks sehingga sulit dilacak jika ada masalah
- untuk sistem yang besar, perlu ada optimisasi tambahan untuk menangani ribuan bidirectional secara bersamaan


4. kelebihan dan kekurangan menggunakan tokio_stream::wrappers::ReceiverStream untuk respons streaming di rust gRPC
kelebihan => penggunaan async/await lebih mudah di tokio. boilerplate yang digunakan untuk mengelola aliran data jadi lebih sedikit. mengurangi risiko kebocoran memori karena channel tokio sudah dioptimalkan untuk konkurensi

kekurangan => pengelolaan laju data dilakukan secara eksplisit dikarenakan tidak ada backpressure bawaan. penggunaan ReceiverStream dapat menambah overhead yang tidak diperlukan dibanding implementasi manual. fleksibilitas seperti menyesuaikan buffer atau logika streaming kurang dapat diterapkan. 


5. struktur kode gRPC yang bisa dipakai kembali dan modular
struktur direktori: 
- pemisahan protos/ untuk file .proto, src/services/ untuk layanan, src/models/ untuk logika bisnis, src/utils. untuk fungsi pendukung, dan src/config untuk konfigurasi. 
- pemisahan logika bisnis lapisan antarmuka dan logika inti
- penggunaan interface layanan untuk memudahkan pengujian dan penggantian implementasi
- buat kode yang dapat digunakan ulang di utils/ seperti autentikasi


6. step tambahan yang diperlukan untuk handle proses payment yang lebih kompleks di MyPaymentService
- validasi input untuk memastikan data yang diterima PaymentRequest valid dan aman sebelum di proses
- pastikan permintaan pembayaran yang sudah pernah diproses tidak diproses lagi
- menghubungkan MyPaymentService dengan layanan pembayaran eksternal seperti ShopeePay, dsb
- menggunakan database untuk manajemen transaksi
- penanganan berbagai skenario error


7. pengaruh penggunaan gRPC sebagai protokol komunikasi dalam segi arsitektur dan design distributed systems
- performa jadi lebih tinggi. penggunaan HTTP/2 pada gRPC mendukung multiplexing dimana beberapa permintaan dan respons dikirim melalui satu koneksi TCP. hal ini dapat mengurangi latensi. penggunaan Protocol Buffers menghasilkan payload lebih kecil dibanding JSON atau XML
- skalabilitas yang lebih besar. HTTP/2 di gRPC menggunakan koneksi peristen dan mengurangi overhead pembukaan koneksi berulang. ini penting untuk menangani ribuan klien dalam sistem mikroservis
- mengurangi boilerplate dan potensi kesalahan karena gRPC menghasilkan kode klien dan server secara otomatis dari file .proto


8. kelebihan dan kekurangan menggunakan HTTP/2 dengan gRPC dibanding HTTP/1.1 dan HTTP/1.1 dengan WebSocket di REST APIs 
dibandingkan dengan HTTP/1.1 => HTTP/2 bisa melakukan multiplexing. penggunaan protobuf lebih cepat dibanding JSON. kekurangannya lebih kompleks karena gRPC perlu konfigurasi TLS, manajemen koneksi persisten, dan tonic. selain itu gRPC tidak didukung langsung oleh browser, perlu grpc-web

dibandingkan dengan HTTP/1.1 dengan WebSocket => gRPC bi-directional streaming lebih ringkas dibanding WebSocket untuk penggunaan komunikasi dua arah. kekurangannya WebSocket memungkinkan protokol kustom yang fleksibel dibanding gRPC yang terikat model RPC dan protobuf. selain itu gRPC perlu backpressure manual seperti menggunakan channel mpsc sedangkan WebSocket sudah memiliki backpressure bawaan. 


9. perbedaan request-response di REST API dengan bidirectional streaming di gRPC dalam segi komunikasi real-time dan responsiveness
REST API => setiap interaksi merupakan transaksi independen dimana klien mengirim permintaan HTTP kemudian server mengembalikan satu respons

bidirectional streaming gRPC => terdapat 2 aliran independen untuk mengirim (server) dan menerima (klien) pesan secara bersamaan. 


10. implikasi menggunakan Protocol Buffers pada gRPC dibanding JSON di REST API
- protobuf menghasilkan data biner yang lebih kecil dan serialisasi/ deserialisasi yang lebih cepat dibandingkan JSON dimana hal ini dapat mengurangi overhead jaringan
- kode dari berbagai bahasa pemrograman dapat dihasilkan dari file .proto
- file .proto mendefisinikan struktur pesan dan layanan secara eksplisit, memastikan kontrak API yang jelas. hal ini mengurangi kesalahan antar klien dan server
- mendukung backward dan forward compatibility. artinya perubahan skema tidak merusak klien lama
- protobuf sulit untuk dibaca langsung tanpa alat debugging seperti protoc
- perlu kompilen protobuf dan plugin seperti tonic untuk menghasilkan kode
