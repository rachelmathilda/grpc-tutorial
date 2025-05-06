1. perbedaan unary, server streaming, dan bi-directional streaming RPC


2. sekuritas yang perlu diconsider ketika menggunakan gRPC berupa: 
- autentikasi
- autorisasi 
- enskripsi data


3. isu dan kesulitan yang mungkin muncul ketika handle bidirectional streaming di rust gRPC


4. kelebihan dan kekurangan menggunakan tokio_stream::wrappers::ReceiverStream untuk respons streaming di rust gRPC


5. struktur kode gRPC yang bisa dipakai kembali dan modular


6. step tambahan yang diperlukan untuk handle proses payment yang lebih kompleks di MyPaymentService


7. pengaruh penggunaan gRPC sebagai protokol komunikasi dalam segi arsitektur dan design distributed systems


8. kelebihan dan kekurangan menggunakan HTTP/2 dengan gRPC dibanding HTTP/1.1 dan HTTP/1.1 dengan WebSocket di REST APIs 


9. perbedaan request-response di REST API dengan bidirectional streaming di gRPC dalam segi komunikasi real-time dan responsiveness


10. implikasi menggunakan Protocol Buffers pada gRPC dibanding JSON di REST API