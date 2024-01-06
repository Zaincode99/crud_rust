use std::io;

#[derive(Debug)]
struct Barang {
    kode_barang: String,
    nama_barang: String,
    jenis_barang: String,
}

fn tambah_data_barang(daftar_data_barang: &mut Vec<Barang>) {
    println!("Tambah Data Barang");

    println!("Masukkan Kode Barang:");
    let mut kode_barang = String::new();
    io::stdin()
        .read_line(&mut kode_barang)
        .expect("Gagal membaca Kode barang");

    println!("Masukkan Nama Barang:");
    let mut nama_barang = String::new();
    io::stdin()
        .read_line(&mut nama_barang)
        .expect("Gagal membaca Nama Barang");

    println!("Masukkan Jenis Barang:");
    let mut jenis_barang = String::new();
    io::stdin()
        .read_line(&mut jenis_barang)
        .expect("Gagal membaca Jenis Barang");

    let data_barang_baru = Barang {
        kode_barang: kode_barang.trim().to_string(),
        nama_barang: nama_barang.trim().to_string(),
        jenis_barang: jenis_barang.trim().to_string(),
    };

    daftar_data_barang.push(data_barang_baru);
    println!("Data Barang berhasil ditambahkan!")
}

fn tampilkan_data_barang(daftar_data_barang: &Vec<Barang>) {
    println!("Data Barang");

    for data_barang in daftar_data_barang {
        println!("{:?}", data_barang);
    }
}

fn main() {
    let mut daftar_data_barang: Vec<Barang> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Tambah Data Barang");
        println!("2. Tampilkan Data Barang");
        println!("3. Keluar");

        let mut pilihan = String::new();
        io::stdin()
            .read_line(&mut pilihan)
            .expect("Pilihan tidak tersedia");

        match pilihan.trim().parse() {
            Ok(1) => tambah_data_barang(&mut daftar_data_barang),
            Ok(2) => tampilkan_data_barang(&daftar_data_barang),
            Ok(3) => {
                println!("Keluar dari program. selamat tinggal!");
                break;
            }
            _ => println!("Pilihan tidak valid. silahkan coba lagi."),
        }
    }
}
