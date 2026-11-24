<p align="center">
  <img src="assets/app-icon/pdfcraft-small.svg" alt="Ikon PdfKu" width="112">
</p>

<h1 align="center">PdfKu</h1>

<p align="center">
  Aplikasi PDF sumber terbuka untuk membaca, menata, mengisi, menandatangani, dan menyunting PDF.<br>
  Berjalan luring, tanpa akun, tanpa telemetri.
</p>

<p align="center">
  <img src="docs/images/pdfku-beranda.png" alt="Layar beranda PdfKu: panel All tools di kiri, kartu tautan proyek, dan alat yang disarankan" width="100%">
</p>

> **Unduh:** installer untuk Windows, macOS, dan Linux ada di halaman
> [Releases](https://github.com/andriyanto/pdfku/releases). Installer belum bertanda tangan digital,
> jadi Windows dan macOS mungkin menampilkan peringatan saat pertama kali dibuka. Anda juga bisa
> [membangun dari kode sumber](#membangun-dari-kode-sumber).

## Apa yang bisa dilakukan

- **Membaca.** Tampilan berkelanjutan, satu halaman, atau dua halaman berdampingan; mode Read
  tanpa gangguan; tema terang dan gelap. Banyak dokumen dibuka sebagai tab dalam satu jendela.
- **Mencari dan menyalin.** Cari teks di dokumen, pilih, lalu salin.
- **Bernavigasi.** Bookmark, gambar mini halaman, dan tautan di dalam dokumen.
- **Menata halaman.** Putar, hapus, pindahkan, sisipkan, dan ekstrak halaman.
- **Menggabung dan memecah.** Gabungkan beberapa berkas menjadi satu, atau pecah satu dokumen.
- **Komentar dan formulir.** Sorotan, catatan, kotak teks, stempel; isi formulir dan Fill & Sign.
- **Tanda tangan digital.** Periksa tanda tangan yang ada dan tanda tangani dengan sertifikat.
- **Menyunting.** Ubah teks dan gambar yang ada, tambahkan header, footer, dan watermark.
- **Keamanan.** Buka dokumen bersandi, lindungi dokumen, dan redaksi isi secara permanen.
- **Pindai dan OCR.** Kenali teks pada halaman hasil pindai supaya bisa dicari.
- **Konversi dan ekspor.** Buat PDF dari gambar dan dokumen lain, ekspor ke format lain, dan
  perkecil ukuran berkas.
- **Pencarian perintah.** Semua alat bisa dicari dan dijalankan lewat `Ctrl+K`.

Antarmuka tersedia dalam bahasa Inggris dan terjemahan ke puluhan bahasa lain; pilih bahasanya di
**Edit → Preferences**.

## Cara PdfKu memperlakukan berkas Anda

- **Simpan tanpa merusak.** Simpan biasa menambahkan perubahan di akhir berkas dan membiarkan isi
  aslinya utuh. Penulisan dilakukan secara atomik, jadi berkas tidak tertinggal setengah tertulis.
- **Tidak ada yang dikirim.** PdfKu tidak mengumpulkan data pemakaian dan tidak mengunggah dokumen.
- **Satu-satunya koneksi keluar bawaan** adalah pemeriksaan versi baru ke GitHub saat aplikasi
  dimulai. Ini bisa dimatikan di **Edit → Preferences** ("Check for updates when PdfKu starts").
- **Tautan dari dokumen** hanya dibuka untuk alamat web dan email, dan PdfKu bertanya dulu.

## Platform

Kode PdfKu ditulis dalam Rust dan dibangun untuk Windows (x64 dan ARM64), macOS, Linux, dan
FreeBSD. Yang sudah diuji otomatis di repo ini adalah Linux dan Windows, termasuk pemasangan
installer di Windows ARM64.

Installer Windows memasang PdfKu ke `Produktivitas\PdfKu`:

| Jenis pemasangan | Folder |
|---|---|
| Hanya untuk saya (bawaan) | `%LOCALAPPDATA%\Programs\Produktivitas\PdfKu` |
| Untuk semua pengguna | `C:\Program Files\Produktivitas\PdfKu` |

## Membangun dari kode sumber

Perlu Rust 1.95 atau yang lebih baru.

```sh
git clone https://github.com/andriyanto/pdfku
cd pdfku
cargo xtask models                           # unduh model OCR untuk Scan & OCR
cargo run --release -p pdfku -- berkas.pdf      # jalankan aplikasi desktop
```

Catatan pengembangan (log, variabel lingkungan, pengujian) ada di
[docs/development.md](docs/development.md). Pustaka di dalam kode masih memakai awalan
`pdfcraft-`; itu nama internal dan tidak tampil ke pengguna.

## Baris perintah dan otomasi

Semua fitur mesin PDF bisa dipanggil tanpa antarmuka grafis lewat `pdfku-cli`:

```sh
pdfku-cli info berkas.pdf                                   # ringkasan dokumen dalam JSON
pdfku-cli text berkas.pdf --page 2                          # teks halaman 2
pdfku-cli combine a.pdf b.pdf --out gabungan.pdf            # gabungkan
pdfku-cli extract berkas.pdf --pages 1,3,5-8 --out hasil.pdf
pdfku-cli tools                                             # daftar semua alat dan skemanya
pdfku-cli run --script langkah.json --root ./kerja          # jalankan beberapa langkah
```

`pdfku-cli mcp` menjalankan server MCP untuk agen AI. Server ini **hanya berjalan bila Anda
menjalankannya sendiri**, berbicara lewat stdin/stdout, dan tidak membuka port jaringan. Dengan
`--root`, semua berkas yang dibaca atau ditulis agen dibatasi ke satu folder.

Rinciannya ada di [crates/automation/README.md](crates/automation/README.md) dan
[docs/mcp.md](docs/mcp.md).

## Komunitas

Tombol **Join the WhatsApp group** di PdfKu membuka bagian ini.

- **Grup WhatsApp:** [Gabung grup komunitas PdfKu](https://chat.whatsapp.com/JpizPcsTktLBWRuWzyg00t) untuk tanya jawab santai dan
  kabar terbaru. Nomor HP Anda terlihat oleh anggota lain di grup, jadi bergabunglah hanya jika
  Anda nyaman dengan itu.
- **[GitHub Discussions](https://github.com/andriyanto/pdfku/discussions):** untuk pertanyaan, ide,
  dan diskusi yang bisa dicari lagi nanti, tanpa membagikan nomor HP.

Laporan masalah sebaiknya tetap lewat [Issues](https://github.com/andriyanto/pdfku/issues), supaya
tidak tenggelam di obrolan.

## Bantuan dan laporan masalah

Laporkan masalah atau usulkan perbaikan di
[Issues](https://github.com/andriyanto/pdfku/issues). Sertakan versi PdfKu (lihat **Help → About
PdfKu**), sistem operasi, dan langkah untuk mengulang masalahnya. Jangan melampirkan dokumen yang
berisi data pribadi.

## Asal-usul dan lisensi

PdfKu adalah versi modifikasi dari [PdfCraft](https://github.com/storytold/pdfcraft) dan
dikembangkan sebagai proyek tersendiri di repo ini. PdfKu tidak dibuat, disponsori, atau didukung
oleh ArtCraft Team; nama dan logo ArtCraft adalah merek dagang mereka dan tidak dipakai di sini.

Kode berlisensi ganda, pilih salah satu:

- [MIT](LICENSE-MIT)
- [Apache License 2.0](LICENSE-APACHE)

Hak cipta (c) 2026 para kontributor PdfKu; bagian yang berasal dari PdfCraft: hak cipta (c) 2026
ArtCraft Team dan para kontributor PdfCraft. Pemberitahuan yang diwajibkan lisensi pihak ketiga
ada di [NOTICE](NOTICE), dan daftar semua aset beserta sumber dan lisensinya ada di
[ATTRIBUTION.md](ATTRIBUTION.md).
