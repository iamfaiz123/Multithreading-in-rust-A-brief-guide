// this code is demonstatre the power of rust ownership and borrow model 
// this code will explain why multitheading is safe in rust

use std::thread::{self, JoinHandle};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub fn multithread_with_copy_trait_types(){

    //to spwan a thread in rust we use thread::spwan,which takes owner ship of the variables of parent function( only if move keyword is used)
    let mut data:i32 = 0;
    let thread1 = thread::spawn(move ||{

    //notice how we're using move keyword to transfer the ownership of the data into the closor, the compiler ask us to declear the data var as static , and 
    //if not the is ask the programmer to pass ownership to the closor as the thread may out live the parent thread.
    //but this might be confusing as we're still using the variable in the parent thread although we passed the ownership to the child thread , how this is ever possible?
    //the i32 data type impls the copy trait, so when we're using the move keyword in the closor what compiler is actully doing is passing a copy of the variable to the closor
    //ownership models helps to overcome the datarace problem as only at a given time a single function can be owner of a varible thus can change its value
          data+=1;
          println!("{data}");
    });

    data+=1;
    println!("{data}");
   
    //even though after awaiting the child thread to finish, the ouput here will be 1 only as the parent and child thread has thier own copy of the variable
    thread1.join().unwrap();
    println!("{data}");
}

pub fn multithread_without_copy_trait_types(){

    #[derive(Debug)]
    struct Apple{
      
        pub number:i32,
     
    }

//we need to put wrap the apple struct instance into a box pointer so that rust does not know its size at compile time
// values whose size are known at compile time can be/will be coppied by closor
 let mut data = Box::new(Apple{number:0});
 

   let thread1 =  thread::spawn(move||{
        
    //the the rust compiler is smart enough,it passes reference and mutable references to the closor depends on the use case
    //if we try to run this code with the move keyword in closor the comppiler will give us an error that the current thread may outlive the parent thread.
    //but since it has a reference to a variable that was define in parent thread,what if the parent thread dies before the child thread?
    //this will produce a dangling pointer in the child thread, which is unsafe
    //so the compiler will ask to declear the passed data as static or pass the ownership to the closor
    /*  
      
    {
         data.number+=1;
    }

    */

   
           
         data.number+=50;

    });

    thread1.join().unwrap();

    //now we have moved ownership to our data to the child thread
    //and if we try to access that in parent thread we will get an error at the compile time itself, thats the power or borrow checker and ownership
    
  /*
     {
       print!("{:?}",data);
     } 
  */
   




}

pub fn multithread_with_message_passing(){
    //now we will see the concepts of channel, Rust uses channels to transfer data from thread to thread.
    //consider this example
    //you have a funnel , the funnel has a single exit ie from its bottom,dosent matter which direction you put the liquid init it will always get outs from its bottom
    //channels are same like funnel , like funnel there is only a single exit point ( reciver ) and can be multiple entering point ( sender )
    //so in short, you can only have a single thread reciving data from a shared instance of channel but multiple thread can used sender of that instance to transfer data to the thread
    //owning the reciver

    //here mpsc stands fro multiple producer and single consumer
    let (data_sender, data_reciver): (Sender<String>, Receiver<String>) = mpsc::channel();

    //we can clone data_sender so that multiple thread can own it and send it to thread owning data_reciver
    // note Receiver<> dosent impls clone trait and its does not make sense either
    
    let mut threads:Vec<JoinHandle<()>> = Vec::new();
    
    // we will create multiple threads and send data from each thread to main thread
    for i in 0..=5{
       let data_sender_clone = data_sender.clone();
       let number = i;
       let local_thread = std::thread::spawn(move||{
                 let local_thread_data = format!("hi i am from thread {number}");
                 data_sender_clone.send(local_thread_data).unwrap();
           }
           
       );
       threads.push(local_thread);
    }
//we will wait for all thread to finish
 for thread in threads{
    thread.join().unwrap();
 }


 // we will iterate through the data stored in reciver and print 
 
 for data in data_reciver{
    println!("{data}");
   }
}
 
