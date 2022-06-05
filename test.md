Rust CLI
```
cargo build --release
./target/release/steganography
```

```rs
Enter path of output image:  
output-2.jpg
Enter path of main image:  
images/mona-lisa.jpg
Enter path of hidden image:  
images/paysage.jpg
Encrypting...
Main Image (642 by 957)
Hidden Image (642 by 488)
Main: [101, 116, 89, 255]
Hidden: [165, 187, 169, 255]
  Enc: [106, 123, 90, 255]
Main: [90, 101, 76, 255]
Hidden: [129, 146, 128, 255]
  Enc: [88, 105, 72, 255]
Main: [99, 105, 83, 255]
Hidden: [142, 150, 132, 255]
  Enc: [104, 105, 88, 255]
Image saved to output-2.jpg
```

```rs
Choose operation ('encrypt' or 'decrypt'):
decrypt
Enter path of output image:  
decrypt-2.jpg
Enter path of main image:  
output-2.jpg
Decrypting...
Main: [105, 119, 94, 255]
  Dec: [144, 112, 224, 255]
Main: [95, 109, 84, 255]
  Dec: [240, 208, 64, 255]
Main: [88, 104, 77, 255]
  Dec: [128, 128, 208, 255]
Image saved to decrypt-2.jpg
```
```rs
// Encrypted and main should be exactly the same
// Difference could be from image encoding
Enc: [106, 123, 90, 255] vs. Main: [105, 119, 94, 255]
Enc: [88, 105, 72, 255] vs. Main: [95, 109, 84, 255]
Enc: [104, 105, 88, 255] vs. Main: [88, 104, 77, 255]
```

```rs
// Hidden and decrypted should be at most +- 16 bits off
// Error because output of enc is not the same as input of dec
Hidden: [165, 187, 169, 255] vs. Dec: [144, 112, 224, 255]
Hidden: [129, 146, 128, 255] vs. Dec: [240, 208, 64, 255]
Hidden: [142, 150, 132, 255] vs. Dec: [128, 128, 208, 255]
```

# Penguin and Mario PNG Test

```rs
Choose operation ('encrypt' or 'decrypt'):
encrypt
Enter path of output image:  
peng-mar-enc.png
Enter path of main image:  
images/penguin.png
Enter path of hidden image:  
images/mario.png
Encrypting...
Main Image (799 by 957)
Hidden Image (799 by 731)
Main: [0, 0, 0, 0]
Hidden: [128, 128, 189, 0]
Enc: [8, 8, 11, 0]
Main: [0, 0, 0, 0]
Hidden: [128, 128, 189, 0]
Enc: [8, 8, 11, 0]
Main: [0, 0, 0, 0]
Hidden: [128, 128, 189, 0]
Enc: [8, 8, 11, 0]
Image saved to peng-mar-enc.png
```

```rs
Main: [8, 8, 11, 0]
Dec: [128, 128, 176, 0]
Main: [8, 8, 11, 0]
Dec: [128, 128, 176, 0]
Main: [8, 8, 11, 0]
Dec: [128, 128, 176, 0]
```