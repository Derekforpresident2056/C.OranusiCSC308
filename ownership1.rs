fn main() {
    let name = String::from("Derek");
    let name2 = "Derek2";
    println!(" ");
    println!("{}",name2);
    println!(" ");
    println!("{}",name);
    println!(" ");

    let x:i32 = 77;
    let y:i32 = x;
    println!("X is {} and Y is {}",x,y);
    println!(" ");

    let s1:String = String::from("hello");
    let s2:String = s1;
    println!("s2 is {}",s2);
    println!(" ");
    // s1 wouldnt print because it doesnt merely copy its value like the earlier example with x and y,
    // it passed its value on, s1 no longer has it
    //This is because when working with string objects, the content of such object, the word itself, is not stored on the stack alongside
    //the pointer, size and memory required. It is stored on a separate heap, this heap is referenced whenever the variable is called
    // now s1 and s2, both string objects are called, they'll point to the same location on the heap. This will cause probelems when they
    // would eventually try to free the same one memory location, this is a 'double free error'. since rust , a frankly rigid but very
    // automated language, would not wish to compromise its data, only the second variable called would be able to access the word stored
    // in the heap, s1 would no longer have a value


     let s3:String = String::from("what's up");
     let s4:String = s3.clone();
     println!("s3 is {} and s4 is {}",s3,s4);
     println!(" ");
     // In the snippet above, both s3 and s4 are valid, They're valid because of the added clause .clone(), This function creates a copy of
     // the heap and so its content and allows it to be assigned to s3, now thwy both have a different memory location to point to and these
     // two subsequent locations could then be freed, this avoids the double free error and allows both be valid.


     let s5:String = String::from("what is up");
     let s6:String = s5;
     let s5:String = String::from("howdy");
     println!("s5 is {} and s6 is {}",s5,s6);
     println!(" ");
     //since we have established that the former defined variable loses its value when the latter is created. When the former defined variable
     //is reassigned to a new value, after the second variable, it now has a new separate value, it points to its own new memory location and 
     // that would mean s5 and s6 would no longer point to the same one anymore, tis would make them both valid

     




    
}