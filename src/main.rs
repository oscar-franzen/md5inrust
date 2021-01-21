// OF; <p.oscar.franzen@gmail.com>
// https://github.com/oscar-franzen/md5inrust

// A vanilla md5 implementation I used as a wau to learn the Rust
// programming language. See README.md for more info on how to
// ccompile.

use std::env;
use std::process;
use std::fs::File;
use std::vec::Vec;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::convert::TryInto;
use std::num::Wrapping;

const BLOCK_SIZE : usize = 32768;

fn rotl32 (value: u64,
	   count: u64) -> u64 {
    (value << count) | (value >> (32 - count))
}

fn tr_f(x: u64,
	y: u64,
	z: u64) -> u64 {
    (x & y) | ( ((!x) & 0xffffffff) & z)
}

fn tr_g(x: u64,
	y: u64,
	z: u64) -> u64 {
    (x & z) | (y & ((!z) & 0xffffffff))
}

fn tr_h(x: u64,
	y: u64,
	z: u64) -> u64 {
    (x ^ y ^ z)
}

fn tr_i(x: u64,
	y: u64,
	z: u64) -> u64 {
    (y ^ (x | ((!z) & 0xffffffff)))
}

fn transform(func: impl Fn(u64, u64, u64) -> u64,
	      a: & u64,
	      b: & u64,
	      c: & u64,
	      d: & u64,
	      word: u32,
	      k: u64,
	      s: u8) -> u64 {

    let mut f = 0u64;
    let mut temp = 0u64;
    
    f = *a + func(*b, *c, *d);
    
    let mut qq : u64 = f as u64 + word as u64;

    qq = qq + k as u64;
    qq = qq & 0xffffffff; // zero out bits over 32
    qq = rotl32(qq, s.into());
    qq = qq & 0xffffffff;
    qq = qq + *b;
    qq & 0xffffffff
}

fn process_block(
    buf: &mut [u8],
    a: &mut u64,
    b: &mut u64,
    c: &mut u64,
    d: &mut u64,
) {
    // divide the block into 512 bit chunks (each chunk consisting
    // of 64 bytes))
    let nchunks = buf.len() / 64;
    
    for i in 0..nchunks {
	//println!("i: {}", i);
	
 	let chunk = &buf[(i*64)..(i*64+64)];
	let mut words : [u32; 16] = [0; 16];

 	// break chunk into 16 words (each word being 32 bits)
 	for ii in 0..16 {
	    let start : usize = ii*4;
	    let stop : usize = ii*4+4;
	    let word : u32 = u32::from_le_bytes(chunk[start..stop].try_into().unwrap());
	    words[ii] = word;
	}
	
	let mut a0 = *a;
	let mut b0 = *b;
	let mut c0 = *c;
	let mut d0 = *d;

	//  round 1, apply on all 16 words
	a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[0], 0xD76AA478, 7);
	d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[1], 0xE8C7B756, 12);
	c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[2], 0x242070DB, 17);
	b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[3], 0xC1BDCEEE, 22);

	a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[4], 0xF57C0FAF, 7);
	d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[5], 0x4787C62A, 12);
	c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[6], 0xA8304613, 17);
	b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[7], 0xFD469501, 22);

	a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[8], 0x698098D8, 7);
	d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[9], 0x8B44F7AF, 12);
	c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[10], 0xFFFF5BB1, 17);
	b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[11], 0x895CD7BE, 22);
	
	a0 = transform(tr_f, &a0, &b0, &c0, &d0, words[12], 0x6B901122, 7);
	d0 = transform(tr_f, &d0, &a0, &b0, &c0, words[13], 0xFD987193, 12);
	c0 = transform(tr_f, &c0, &d0, &a0, &b0, words[14], 0xA679438E, 17);
	b0 = transform(tr_f, &b0, &c0, &d0, &a0, words[15], 0x49B40821, 22);
	
	// round 2, apply on all 16 words
	a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[1], 0xF61E2562, 5);
	d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[6], 0xC040B340, 9);
	c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[11], 0x265E5A51, 14);
	b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[0], 0xE9B6C7AA, 20);

	a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[5], 0xD62F105D, 5);
	d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[10], 0x02441453, 9);
	c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[15], 0xD8A1E681, 14);
	b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[4], 0xE7D3FBC8, 20);

	a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[9], 0x21E1CDE6, 5);
	d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[14], 0xC33707D6, 9);
	c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[3], 0xF4D50D87, 14);
	b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[8], 0x455A14ED, 20);

	a0 = transform(tr_g, &a0, &b0, &c0, &d0, words[13], 0xA9E3E905, 5);
	d0 = transform(tr_g, &d0, &a0, &b0, &c0, words[2], 0xFCEFA3F8, 9);
	c0 = transform(tr_g, &c0, &d0, &a0, &b0, words[7], 0x676F02D9, 14);
	b0 = transform(tr_g, &b0, &c0, &d0, &a0, words[12], 0x8D2A4C8A, 20);

	// round 3, apply on all 16 words
	a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[5], 0xFFFA3942, 4);
	d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[8], 0x8771F681, 11);
	c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[11], 0x6D9D6122, 16);
	b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[14], 0xFDE5380C, 23);

	a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[1], 0xA4BEEA44, 4);
	d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[4], 0x4BDECFA9, 11);
	c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[7], 0xF6BB4B60, 16);
	b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[10], 0xBEBFBC70, 23);

	a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[13], 0x289B7EC6, 4);
	d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[0], 0xEAA127FA, 11);
	c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[3], 0xD4EF3085, 16);
	b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[6], 0x04881D05, 23);

	a0 = transform(tr_h, &a0, &b0, &c0, &d0, words[9], 0xD9D4D039, 4);
	d0 = transform(tr_h, &d0, &a0, &b0, &c0, words[12], 0xE6DB99E5, 11);
	c0 = transform(tr_h, &c0, &d0, &a0, &b0, words[15], 0x1FA27CF8, 16);
	b0 = transform(tr_h, &b0, &c0, &d0, &a0, words[2], 0xC4AC5665, 23);

	// round 4, apply on all 16 words
	a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[0], 0xF4292244, 6);
	d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[7], 0x432AFF97, 10);
	c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[14], 0xAB9423A7, 15);
	b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[5], 0xFC93A039, 21);

	a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[12], 0x655B59C3, 6);
	d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[3], 0x8F0CCC92, 10);
	c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[10], 0xFFEFF47D, 15);
	b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[1], 0x85845DD1, 21);

	a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[8], 0x6FA87E4F, 6);
	d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[15], 0xFE2CE6E0, 10);
	c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[6], 0xA3014314, 15);
	b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[13], 0x4E0811A1, 21);

	a0 = transform(tr_i, &a0, &b0, &c0, &d0, words[4], 0xF7537E82, 6);
	d0 = transform(tr_i, &d0, &a0, &b0, &c0, words[11], 0xBD3AF235, 10);
	c0 = transform(tr_i, &c0, &d0, &a0, &b0, words[2], 0x2AD7D2BB, 15);
	b0 = transform(tr_i, &b0, &c0, &d0, &a0, words[9], 0xEB86D391, 21);

	// add to results
	*a = (*a + a0) & 0xffffffff;
	*b = (*b + b0) & 0xffffffff;
	*c = (*c + c0) & 0xffffffff;
	*d = (*d + d0) & 0xffffffff;
    }
}

fn main() {   
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./md5sum <filename>");
        process::exit(0);
    }

    let file = &args[1];
    
    let mut a : u64 = 0x67452301;
    let mut b : u64 = 0xefcdab89;
    let mut c : u64 = 0x98badcfe;
    let mut d : u64 = 0x10325476;

    let fh = File::open(&file);
    let mut reader = BufReader::new(fh.unwrap());
    
    println!("Loading {}...", file);

    let mut buf = [0; BLOCK_SIZE];
    let mut count = 0;

    // https://www.ietf.org/rfc/rfc1321.txt
	
    loop {
	// read blocks (32 kb)
	let res = reader.read(&mut buf).unwrap();

	if res == 0 {
	    break;
	}

	count += res;

	// this is the last block
	if res != BLOCK_SIZE {
	    // how many bytes are missing from a complete 64 byte
	    // multiple?
	    let size = 64 - (res as i32 % 64);
	    let mut padding_to_add : usize = 0;

	    // we do like this b/c: "as many zeros as are required to
	    // bring the length of the message up to 64 bits (8
	    // bytes) fewer than a multiple of 512 (64 bytes)"

	    if (size - 8) < 0 {
		padding_to_add = size as usize + (64 - 8);
	    } else {
		padding_to_add = size as usize - 8;
	    }
	    
	    let mut padbuf = vec![0; padding_to_add];
	    padbuf[0] = 0x80;

	    // The remaining bits are filled up with 64 bits
	    // representing the length of the original message in
	    // *BITS* (not bytes), modulo 2^64.

	    let mut bitsize = Wrapping(count as u64*8);
	    let mut orig_size : [u8; 8] = bitsize.0.to_le_bytes();
	    let mut orig_size = orig_size.to_vec();
	    padbuf.append(&mut orig_size);

	    let mut buf2 : Vec<u8> = Vec::new();

	    buf2.append(&mut buf[0..res].to_vec());
	    buf2.append(&mut padbuf.to_vec());

	    process_block(&mut buf2, &mut a, &mut b, &mut c, &mut d);
	}
	else {
	    process_block(&mut buf, &mut a, &mut b, &mut c, &mut d);
	}
    }

    // print results
    let bytes1 : [u8; 4] = (a as u32).to_le_bytes();
    let bytes2 : [u8; 4] = (b as u32).to_le_bytes();
    let bytes3 : [u8; 4] = (c as u32).to_le_bytes();
    let bytes4 : [u8; 4] = (d as u32).to_le_bytes();

    println!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}\
{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
	     bytes1[0], bytes1[1], bytes1[2], bytes1[3],
	     bytes2[0], bytes2[1], bytes2[2], bytes2[3],
	     bytes3[0], bytes3[1], bytes3[2], bytes3[3],
	     bytes4[0], bytes4[1], bytes4[2], bytes4[3]
    );

    //println!("F: {}", F);
    //let _ = io::stdin().read(&mut [0u8]).unwrap();
}
