# little-rusty

This is an experimental http file server built using rust.

## How to build
* Clone the repository and switch directories:  
  `git clone git@github.com:dafaqdhruv/little-rusty.git && cd little-rusty`
* To build the binary, type :  
  `cargo build`
  
 ## Setting up the server
* On the directory you want to set up the server, type :
  ```
  ./path/to/binary <port number>
  ```
  Example `./little-rusty 8000` starts the server which listens at port 8000.

You can now access your directory from just a web browser!  
Just type in the local IP address of your device suffixed with the port number. 


## Note :
This is an unsafe implementation as it starts the server on `0.0.0.0` instead of `127.0.0.1` which opens your directory to the local network.  
If this is not something you want, change the bind address in `main()` to `LOCAL_PRIVATE_IP`.
