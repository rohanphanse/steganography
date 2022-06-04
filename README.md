# Steganography

Steganography is a form of cryptography where a hidden message is concealed inside another message. For images, this process works by encrypting the most significant bits of a hidden image into the least significant bits of a main image, and then displaying the hidden image bits to reveal the image. The beauty of the steganography is that although the hidden bits fly under the human vision radar (in theory, my encrypter is too generous and wants both images to shine 🙂), the information is readily available for the computer to decrypt it.

## Features
* Encrypt a hidden image into another image
* Decrypt and reveal hidden image
* Beautiful terminal interface with colors!
* Automatically scale images to fit correctly
* Bit manipulations on image buffer 😋

## Development Process

I wrote this program in Rust and gained experience with using the `image` crate from this amazing tutorial by freeCodeCamp ([link](https://www.freecodecamp.org/news/rust-in-replit/)).

## Featured Code

I want to feature 2 pieces of code, because they were the most rewarding and challenging things to get right.

**1. Bit Manipulation**

This was super fun for me to figure out and I learned how to use the bitwise operators `AND &`, `OR |`, and `<< BITSHIFT >>`.

```rs
// src/main.rs, line 129
encrypted.push(
    (main_pixel[i] & 0b_1100_0000) +        // Replace last 6 bits of main pixel
    ((hidden_pixel[i] & 0b_1111_1100) >> 2) // with first 6 bits of hidden pixel
);
```

**2. Traversing Through Image Buffer**

Images are stored in buffers (1d arrays), so I had to figure out how to traverse the buffer while keeping track of the 2d location of the pixels. After a lot of mindless debugging, I realized that `image_height * image_width * 4 == buffer_length` and having reached enlightenment, I blissfully wrote the code below in a state of nirvana.

```rs
// src/main.rs, lines 95-104
for h in 0..main_height {
    for w in 0..main_width {
        i = (h * main_width + w) * 4;
        if h < hidden_height && w < hidden_width  {
            encrypted.splice(i..=i + 3, encrypt_bits(&main_vec, i, &hidden_vec, (h * hidden_width + w) * 4));
        } else {
            encrypted.splice(i..=i + 3, lose_bits(&main_vec, i));
        }
    }
}
```


## Examples

Here are 2 examples of my steganography encrypter / decrypter.

Main | Hidden
---- | ----
![main image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/paysage.jpg) | ![hidden image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/mona-lisa.jpg)

*Credit to Wikipedia for [Landscape](https://commons.wikimedia.org/wiki/File:Camille_Pissarro_-_Paisaje_tropical.jpg) and [Mona Lisa](https://en.wikipedia.org/wiki/File:Mona_Lisa.jpg) images*

Encrypted | Decrypted
----  | ----
![main image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/aaa.jpg) | ![hidden image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/ddd.jpg)

Main | Hidden
---- | ----
![main image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/mona-lisa.jpg) | ![hidden image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/bday.jpg)

*Credit to [Ruth Black](https://www.dreamstime.com/happy-birthday-cupcake-celebration-message-image160558421) for Birthday image*


Encrypted | Decrypted
----  | ----
![main image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/bday-enc.jpg) | ![hidden image](https://raw.githubusercontent.com/rohanphanse/steganography/main/images/bday-dec.jpg)