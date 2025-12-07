use std :: thread;

fn main() {
	let message1 = String :: from("Thread says 1 hello!");
	let message2 = String :: from("Thread says 2 hello!");
	let message3 = String :: from("Thread says 3 hello!");
	let message4 = String :: from("Thread says 4 hello!");
	let message5 = String :: from("Thread says 5 hello!");

	let m1_1 = message1.clone();
	let m1_2 = message2.clone();
	let m1_3 = message3.clone();
	let m1_4 = message4.clone();
	let m1_5 = message5.clone();

	let m2_1 = message1.clone();
	let m2_2 = message2.clone();
	let m2_3 = message3.clone();
	let m2_4 = message4.clone();
	let m2_5 = message5.clone();

	let thread1 = thread:: spawn (move || {
		let tnum = "Thread 1: ";
		println! ("{} {}",tnum, message1);
		println! ("{} {}",tnum, message2);
		println! ("{} {}",tnum, message3);
		println! ("{} {}",tnum, message4);
		println! ("{} {}",tnum, message5);
		println!("");
	});


	let thread2 = thread:: spawn (move || {
		let tnum = "Thread 2: ";
		println! ("{} {}",tnum, m1_1);
		println! ("{} {}",tnum, m1_2);
		println! ("{} {}",tnum, m1_3);
		println! ("{} {}",tnum, m1_4);
		println! ("{} {}",tnum, m1_5);
		println!("");
	});


	let thread3 = thread:: spawn (move || {
		let tnum = "Thread 3: ";
		println! ("{} {}",tnum, m2_1);
		println! ("{} {}",tnum, m2_2);
		println! ("{} {}",tnum, m2_3);
		println! ("{} {}",tnum, m2_4);
		println! ("{} {}",tnum, m2_5);
		println!("");
	});

	thread1.join().unwrap();
	thread2.join().unwrap();
	thread3.join().unwrap();
}